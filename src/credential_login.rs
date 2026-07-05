//! Website credential exchange for DriveThruRPG.
//!
//! This module targets `www.drivethrurpg.com` — **not** `api.drivethrurpg.com`.
//! It wraps the two website login endpoints that DriveThruRPG's own login page
//! uses to turn an email/password pair into an application key.
//!
//! This is distinct from [`auth_client`], which exchanges an application key for
//! a short-lived JWT against `api.drivethrurpg.com`. [`login_with_credentials`]
//! produces the application key that [`auth_client::authenticate`] then exchanges
//! for a session token; the two modules are complementary, not overlapping.
//! `auth_client::authenticate` is unaffected by this module.
//!
//! [`auth_client`]: crate::auth_client
//! [`auth_client::authenticate`]: crate::auth_client::authenticate

use serde::Deserialize;

use crate::{client::ClientError, config::Config};

/// Base URL for the DriveThruRPG website login endpoints.
const WEBSITE_BASE_URL: &str = "https://www.drivethrurpg.com";

/// Maximum number of bytes preserved in a decode-failure log payload.
const LOG_PAYLOAD_LIMIT: usize = 2_000;

// ── Response types ────────────────────────────────────────────────────────────

/// Typed response from `POST /validate_login_credentials.php`.
///
/// The endpoint returns a bare JSON array (not an object). Field order per
/// `dtrpg-api/LOGIN.md`: `[field_name, ok, message, locked]`.
///
/// Example: `["password", true, "Locked", true]`
#[derive(Debug, PartialEq)]
pub struct ValidateLoginResponse {
    /// The field that was validated (e.g. `"password"`).
    pub field_name: String,
    /// Whether the credentials are valid.
    pub ok: bool,
    /// Status message from the server (e.g. `"Locked"`).
    pub message: String,
    /// Whether the account is locked.
    pub locked: bool,
}

impl<'de> Deserialize<'de> for ValidateLoginResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Vis;

        impl<'de> serde::de::Visitor<'de> for Vis {
            type Value = ValidateLoginResponse;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("a 4-element JSON array [field_name, ok, message, locked]")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let field_name = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let ok = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                let message = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;
                let locked = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(3, &self))?;
                Ok(ValidateLoginResponse {
                    field_name,
                    ok,
                    message,
                    locked,
                })
            }
        }

        deserializer.deserialize_seq(Vis)
    }
}

/// Typed response from `POST /create_account_app.php`.
#[derive(Debug, Deserialize)]
struct CreateAccountAppResponse {
    status: String,
    message: CreateAccountAppMessage,
}

/// Inner `message` object from `create_account_app.php`.
#[derive(Debug, Deserialize)]
struct CreateAccountAppMessage {
    key: String,
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Exchanges an email/password pair for a DriveThruRPG application key.
///
/// Calls `POST /validate_login_credentials.php` on `www.drivethrurpg.com`. If
/// credentials are valid, calls `POST /create_account_app.php` and returns the
/// application key from `message.key`. Both requests use `multipart/form-data`
/// with `email_address` and `password` fields, per `dtrpg-api/LOGIN.md`.
///
/// # Errors
///
/// - [`ClientError::Http`] on any transport or server error.
/// - [`ClientError::InvalidCredentials`] if `validate_login_credentials.php`
///   indicates the credentials are invalid.
/// - [`ClientError::ApplicationKeyRequestFailed`] if credentials pass validation
///   but `create_account_app.php` returns a non-success status.
/// - [`ClientError::DecodeFailed`] if a response body cannot be parsed.
///
/// # Examples
///
/// ```rust,no_run
/// use dtrpg_sdk::{Config, credential_login};
///
/// async fn get_key() -> Result<String, dtrpg_sdk::ClientError> {
///     let config = Config::new("placeholder");
///     credential_login::login_with_credentials("user@example.com", "secret", &config).await
/// }
/// ```
pub async fn login_with_credentials(
    email: &str,
    password: &str,
    _config: &Config,
) -> Result<String, ClientError> {
    do_login(email, password, WEBSITE_BASE_URL).await
}

async fn do_login(email: &str, password: &str, base_url: &str) -> Result<String, ClientError> {
    let http = reqwest::Client::new();

    // Step 1: validate credentials
    let validate_url = format!("{base_url}/validate_login_credentials.php");
    tracing::debug!(method = "POST", url = %validate_url, "SDK request");

    let form = reqwest::multipart::Form::new()
        .text("email_address", email.to_owned())
        .text("password", password.to_owned());

    let validate_resp = http.post(&validate_url).multipart(form).send().await?;
    let validate_status = validate_resp.status().as_u16();
    tracing::debug!(url = %validate_url, status = validate_status, "SDK response");

    let validate_bytes = validate_resp.bytes().await.map_err(ClientError::Http)?;
    let validated =
        serde_json::from_slice::<ValidateLoginResponse>(&validate_bytes).map_err(|cause| {
            let raw = String::from_utf8_lossy(&validate_bytes);
            let payload = truncated_payload(&raw);
            tracing::error!(
                url = %validate_url,
                status = validate_status,
                payload = %payload,
                error = %cause,
                "validate_login_credentials response decode failed"
            );
            ClientError::DecodeFailed {
                url: validate_url.clone(),
                status: validate_status,
                cause,
                payload,
            }
        })?;

    if !validated.ok {
        tracing::debug!(url = %validate_url, "credential validation rejected");
        return Err(ClientError::InvalidCredentials);
    }

    // Step 2: request the application key
    let key_url = format!("{base_url}/create_account_app.php");
    tracing::debug!(method = "POST", url = %key_url, "SDK request");

    let form = reqwest::multipart::Form::new()
        .text("email_address", email.to_owned())
        .text("password", password.to_owned());

    let key_resp = http.post(&key_url).multipart(form).send().await?;
    let key_status = key_resp.status().as_u16();
    tracing::debug!(url = %key_url, status = key_status, "SDK response");

    let key_bytes = key_resp.bytes().await.map_err(ClientError::Http)?;
    let key_result =
        serde_json::from_slice::<CreateAccountAppResponse>(&key_bytes).map_err(|cause| {
            let raw = String::from_utf8_lossy(&key_bytes);
            let payload = truncated_payload(&raw);
            tracing::error!(
                url = %key_url,
                status = key_status,
                payload = %payload,
                error = %cause,
                "create_account_app response decode failed"
            );
            ClientError::DecodeFailed {
                url: key_url.clone(),
                status: key_status,
                cause,
                payload,
            }
        })?;

    if key_result.status != "success" {
        tracing::error!(
            url = %key_url,
            status = %key_result.status,
            "create_account_app returned non-success status"
        );
        return Err(ClientError::ApplicationKeyRequestFailed {
            status: key_result.status,
        });
    }

    Ok(key_result.message.key)
}

fn truncated_payload(raw: &str) -> String {
    if raw.len() > LOG_PAYLOAD_LIMIT {
        format!("{}... (truncated)", &raw[..LOG_PAYLOAD_LIMIT])
    } else {
        raw.to_owned()
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    // ── ValidateLoginResponse unit tests ──────────────────────────────────────

    #[test]
    fn validate_login_response_parses_example_from_login_md() {
        // Exact example from dtrpg-api/LOGIN.md
        let json = r#"["password",true,"Locked",true]"#;
        let response: ValidateLoginResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.field_name, "password");
        assert!(response.ok);
        assert_eq!(response.message, "Locked");
        assert!(response.locked);
    }

    #[test]
    fn validate_login_response_parses_invalid_credentials() {
        let json = r#"["password",false,"Invalid",false]"#;
        let response: ValidateLoginResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.field_name, "password");
        assert!(!response.ok);
        assert_eq!(response.message, "Invalid");
        assert!(!response.locked);
    }

    #[test]
    fn validate_login_response_rejects_short_array() {
        let json = r#"["password",true]"#;
        assert!(serde_json::from_str::<ValidateLoginResponse>(json).is_err());
    }

    // ── Integration-style tests using a local HTTP server ─────────────────────

    #[tokio::test]
    async fn valid_credentials_return_application_key() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/validate_login_credentials.php"))
            .respond_with(
                ResponseTemplate::new(200).set_body_string(r#"["password",true,"Locked",true]"#),
            )
            .expect(1)
            .mount(&server)
            .await;

        Mock::given(method("POST"))
            .and(path("/create_account_app.php"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "success",
                "message": { "key": "test-app-key-abc123" }
            })))
            .expect(1)
            .mount(&server)
            .await;

        let result = do_login("user@example.com", "correct-password", &server.uri()).await;
        assert_eq!(result.unwrap(), "test-app-key-abc123");
    }

    #[tokio::test]
    async fn invalid_credentials_return_error_without_calling_key_endpoint() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/validate_login_credentials.php"))
            .respond_with(
                ResponseTemplate::new(200).set_body_string(r#"["password",false,"Invalid",false]"#),
            )
            .expect(1)
            .mount(&server)
            .await;

        // Mounted with expect(0) — verifies the key endpoint is never called.
        Mock::given(method("POST"))
            .and(path("/create_account_app.php"))
            .respond_with(ResponseTemplate::new(200))
            .expect(0)
            .mount(&server)
            .await;

        let result = do_login("user@example.com", "wrong-password", &server.uri()).await;
        assert!(
            matches!(result, Err(ClientError::InvalidCredentials)),
            "expected InvalidCredentials, got: {result:?}"
        );
    }

    #[tokio::test]
    async fn key_request_failure_after_valid_credentials_is_distinguishable() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/validate_login_credentials.php"))
            .respond_with(
                ResponseTemplate::new(200).set_body_string(r#"["password",true,"Locked",true]"#),
            )
            .expect(1)
            .mount(&server)
            .await;

        Mock::given(method("POST"))
            .and(path("/create_account_app.php"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "status": "error",
                "message": { "key": "" }
            })))
            .expect(1)
            .mount(&server)
            .await;

        let result = do_login("user@example.com", "correct-password", &server.uri()).await;
        assert!(
            matches!(
                result,
                Err(ClientError::ApplicationKeyRequestFailed { ref status }) if status == "error"
            ),
            "expected ApplicationKeyRequestFailed, got: {result:?}"
        );
    }
}
