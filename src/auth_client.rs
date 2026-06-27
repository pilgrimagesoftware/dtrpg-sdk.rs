//! Async HTTP client for the DriveThruRPG authentication endpoint.
//!
//! [`authenticate`] exchanges a DriveThruRPG application key for a short-lived JWT
//! access token and a long-lived refresh token. It is the only SDK operation that does
//! not require a pre-existing [`AuthSession`].
//!
//! [`AuthSession`]: crate::AuthSession

use crate::{auth::AuthTokenResponse, client::ClientError, config::Config};

/// Maximum number of bytes logged from a failing auth response body.
const LOG_PAYLOAD_LIMIT: usize = 2_000;

/// Exchanges a DriveThruRPG application key for a session token.
///
/// Posts to `POST /{api_version}/auth_key` with `applicationKey` as a query parameter.
/// On success, returns the JWT access token, refresh token, and refresh token TTL.
///
/// # Errors
///
/// Returns [`ClientError::Http`] on transport or server errors.
/// Returns [`ClientError::DecodeFailed`] if the response body cannot be parsed.
pub async fn authenticate(api_key: &str, config: &Config) -> Result<AuthTokenResponse, ClientError> {
    let url = format!("{}/{}/auth_key", config.base_url(), config.api_version());
    let http = reqwest::Client::new();

    tracing::debug!(method = "POST", url = %url, "SDK request");
    let response = http
        .post(&url)
        .query(&[("applicationKey", api_key)])
        .json(&serde_json::json!({}))
        .send()
        .await?;
    let status = response.status().as_u16();
    tracing::debug!(url = %url, status = status, "SDK response");

    let bytes = response.bytes().await.map_err(ClientError::Http)?;
    serde_json::from_slice::<AuthTokenResponse>(&bytes).map_err(|cause| {
        let raw = String::from_utf8_lossy(&bytes);
        let payload: String = if raw.len() > LOG_PAYLOAD_LIMIT {
            format!("{}… (truncated)", &raw[..LOG_PAYLOAD_LIMIT])
        } else {
            raw.into_owned()
        };
        tracing::error!(
            url = %url,
            status = status,
            payload = %payload,
            error = %cause,
            "auth_key response decode failed"
        );
        ClientError::DecodeFailed { url: url.clone(), status, cause, payload }
    })
}
