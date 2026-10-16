//! Async HTTP client for the DriveThruRPG token refresh endpoint.
//!
//! [`refresh`] exchanges a refresh token for a new access token, so a session can be
//! extended without repeating the application-key exchange.
//!
//! The refresh endpoint is not described by the bundled OpenAPI document. Its shape is
//! inferred from the `Set-Cookie` header on the `auth_key` response, which scopes the
//! `refreshToken` cookie to `/api/vBeta/auth/refresh`: the token is sent back as that
//! cookie, and the response is assumed to carry the same payload as `auth_key`.

use crate::{
    auth::{AuthTokenResponse, key_exchange::decode_token_response},
    config::Config,
    library::ClientError,
};

/// Exchanges a refresh token for a new access token.
///
/// Posts to `POST /{api_version}/auth/refresh` with the token in a `refreshToken`
/// cookie. On success, returns a new access token together with the refresh token and
/// expiry the API now considers current; callers should store all three, because the
/// API may rotate the refresh token.
///
/// # Errors
///
/// Returns [`ClientError::Http`] on transport errors.
/// Returns [`ClientError::DecodeFailed`] if the response body cannot be parsed, which
/// includes an error response such as `401` for an expired or revoked refresh token.
///
/// # Examples
///
/// ```rust,no_run
/// # use dtrpg_sdk::{Config, auth::token_refresh};
/// # async fn run() -> Result<(), dtrpg_sdk::ClientError> {
/// let config = Config::new("my-app-key");
/// let response = token_refresh::refresh("my-refresh-token", &config).await?;
/// println!("new access token: {}", response.token);
/// # Ok(())
/// # }
/// ```
pub async fn refresh(
    refresh_token: &str,
    config: &Config,
) -> Result<AuthTokenResponse, ClientError> {
    let url = format!(
        "{}/{}/auth/refresh",
        config.base_url(),
        config.api_version()
    );
    let http = reqwest::Client::new();

    tracing::debug!(method = "POST", url = %url, "SDK request");
    let response = http
        .post(&url)
        .header(
            reqwest::header::COOKIE,
            format!("refreshToken={refresh_token}"),
        )
        .json(&serde_json::json!({}))
        .send()
        .await?;
    let status = response.status().as_u16();
    tracing::debug!(url = %url, status = status, "SDK response");

    let bytes = response.bytes().await.map_err(ClientError::Http)?;
    decode_token_response(&url, status, &bytes)
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    const TOKEN_BODY: &str =
        r#"{"token":"new-jwt","refreshToken":"new-refresh","refreshTokenTTL":2000000000}"#;

    fn config_for(server: &MockServer) -> Config {
        Config::with_base_url("app-key", server.uri())
    }

    #[tokio::test]
    async fn refresh_sends_token_as_cookie_and_decodes_response() {
        let server = MockServer::start().await;
        let config = config_for(&server);
        let expected_path = format!("/{}/auth/refresh", config.api_version());
        Mock::given(method("POST"))
            .and(path(expected_path))
            .and(header("cookie", "refreshToken=old-refresh"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(TOKEN_BODY, "application/json"))
            .expect(1)
            .mount(&server)
            .await;

        let response = refresh("old-refresh", &config).await.unwrap();

        assert_eq!(response.token, "new-jwt");
        assert_eq!(response.refresh_token, "new-refresh");
        assert_eq!(response.refresh_token_ttl, 2_000_000_000);
    }

    #[tokio::test]
    async fn refresh_reports_rejected_token_as_decode_failure_with_status() {
        let server = MockServer::start().await;
        let config = config_for(&server);
        Mock::given(method("POST"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_raw(r#"{"errorCode":"refresh_expired"}"#, "application/json"),
            )
            .mount(&server)
            .await;

        let error = refresh("expired", &config).await.unwrap_err();

        assert!(matches!(
            error,
            ClientError::DecodeFailed { status: 401, .. }
        ));
    }
}
