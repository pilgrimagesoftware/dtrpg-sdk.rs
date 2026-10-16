//! The primary SDK entry point.
//!
//! [`DriveThruRpgSdk`] is the root object that holds SDK configuration and manages the
//! active authentication session. Start here when integrating the DriveThruRPG API into
//! your application.

use crate::{
    auth::{AuthSession, AuthTokenResponse, token_refresh},
    config::Config,
    error::{AuthSessionError, SdkError},
    library::ClientError,
};

/// The DriveThruRPG SDK client.
///
/// `DriveThruRpgSdk` coordinates SDK-level configuration and authentication session
/// lifecycle. It must be configured before any authenticated API calls can succeed.
///
/// # Lifecycle
///
/// 1. Create an SDK instance, optionally supplying [`Config`] upfront.
/// 2. After a successful API login, call [`apply_auth_response`] to store the session.
/// 3. Use [`require_session`] to obtain the active session before making requests.
/// 4. Call [`invalidate_session`] when the API reports a session error, or
///    [`clear_session`] to log out.
///
/// # Examples
///
/// ```rust
/// use dtrpg_sdk::{Config, DriveThruRpgSdk, AuthTokenResponse};
///
/// let mut sdk = DriveThruRpgSdk::with_config(Config::new("my-app-key"));
/// let response = AuthTokenResponse::new("jwt", "refresh", 9_999_999_999);
/// let session = sdk.apply_auth_response(response).unwrap();
/// assert_eq!(session.token(), "jwt");
/// ```
///
/// [`apply_auth_response`]: DriveThruRpgSdk::apply_auth_response
/// [`require_session`]: DriveThruRpgSdk::require_session
/// [`invalidate_session`]: DriveThruRpgSdk::invalidate_session
/// [`clear_session`]: DriveThruRpgSdk::clear_session
#[derive(Clone, Debug, Default)]
pub struct DriveThruRpgSdk {
    config: Option<Config>,
    session: Option<AuthSession>,
}

impl DriveThruRpgSdk {
    /// Creates an unconfigured SDK instance.
    ///
    /// Call [`configure`] before making any API calls, or prefer [`with_config`] if the
    /// configuration is available at construction time.
    ///
    /// [`configure`]: DriveThruRpgSdk::configure
    /// [`with_config`]: DriveThruRpgSdk::with_config
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an SDK instance pre-loaded with the given [`Config`].
    pub fn with_config(config: Config) -> Self {
        Self {
            config: Some(config),
            session: None,
        }
    }

    /// Sets or replaces the SDK's configuration.
    ///
    /// This can be called at any time, including after the SDK has been used. Replacing
    /// the configuration does not automatically clear an existing session.
    pub fn configure(&mut self, config: Config) {
        self.config = Some(config);
    }

    /// Returns the current configuration, or `None` if the SDK is unconfigured.
    pub fn config(&self) -> Option<&Config> {
        self.config.as_ref()
    }

    /// Returns the current authentication session, or `None` if unauthenticated.
    pub fn session(&self) -> Option<&AuthSession> {
        self.session.as_ref()
    }

    /// Returns the current configuration, or [`SdkError::Unconfigured`] if absent.
    ///
    /// Use this in call chains where a missing config should surface as an error.
    pub fn require_config(&self) -> Result<&Config, SdkError> {
        self.config.as_ref().ok_or(SdkError::Unconfigured)
    }

    /// Returns the current session, or [`SdkError::Unauthenticated`] if absent.
    ///
    /// Use this in call chains where a missing session should surface as an error.
    pub fn require_session(&self) -> Result<&AuthSession, SdkError> {
        self.session.as_ref().ok_or(SdkError::Unauthenticated)
    }

    /// Stores a new authentication session derived from a raw API token response.
    ///
    /// Returns [`SdkError::Unconfigured`] if the SDK has not been configured yet.
    /// On success, returns a reference to the newly stored session.
    pub fn apply_auth_response(
        &mut self,
        response: AuthTokenResponse,
    ) -> Result<&AuthSession, SdkError> {
        self.require_config()?;
        self.session = Some(AuthSession::from_api_response(response));
        self.require_session()
    }

    /// Renews the access token using the active session's refresh token.
    ///
    /// On success the stored session is replaced with the one the API returns,
    /// including its (possibly rotated) refresh token and expiry, and a reference to it
    /// is returned. Any [`LibraryClient`] created earlier still holds the old access
    /// token; call [`library_client`] again to get one with the new token.
    ///
    /// On failure the existing session is left in place, so the caller can decide
    /// whether to retry, fall back to a fresh key exchange, or [`clear_session`].
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Sdk`] if the SDK is unconfigured or has no session.
    /// Returns [`ClientError::Http`] on transport errors, and
    /// [`ClientError::DecodeFailed`] if the API rejects the refresh token.
    ///
    /// [`LibraryClient`]: crate::LibraryClient
    /// [`library_client`]: DriveThruRpgSdk::library_client
    /// [`clear_session`]: DriveThruRpgSdk::clear_session
    pub async fn refresh_session(&mut self) -> Result<&AuthSession, ClientError> {
        let config = self.require_config()?;
        let refresh_token = self.require_session()?.refresh_token();
        let response = token_refresh::refresh(refresh_token, config).await?;
        Ok(self.apply_auth_response(response)?)
    }

    /// Removes the current authentication session without recording an error.
    ///
    /// Use this for voluntary log-out flows. For API-reported session failures,
    /// prefer [`invalidate_session`].
    ///
    /// [`invalidate_session`]: DriveThruRpgSdk::invalidate_session
    pub fn clear_session(&mut self) {
        self.session = None;
    }

    /// Removes the current session and records the API-reported error that caused it.
    ///
    /// Returns [`SdkError::Unauthenticated`] if there is no session to invalidate.
    /// On success, returns the provided `error` so callers can inspect or propagate it.
    pub fn invalidate_session(
        &mut self,
        error: AuthSessionError,
    ) -> Result<AuthSessionError, SdkError> {
        self.require_session()?;
        self.clear_session();
        Ok(error)
    }

    /// Creates a [`LibraryClient`] from the current configuration and session.
    ///
    /// Returns [`SdkError::Unconfigured`] if the SDK has not been configured, or
    /// [`SdkError::Unauthenticated`] if there is no active session.
    ///
    /// [`LibraryClient`]: crate::LibraryClient
    pub fn library_client(&self) -> Result<crate::library::LibraryClient, crate::error::SdkError> {
        let config = self.require_config()?.clone();
        let token = self.require_session()?.token().to_string();
        Ok(crate::library::LibraryClient::new(config, token))
    }
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{header, method};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    const REFRESHED_BODY: &str =
        r#"{"token":"new-jwt","refreshToken":"new-refresh","refreshTokenTTL":2000000000}"#;

    fn sdk_with_session(server: &MockServer) -> DriveThruRpgSdk {
        let mut sdk = DriveThruRpgSdk::with_config(Config::with_base_url("app-key", server.uri()));
        sdk.apply_auth_response(AuthTokenResponse::new(
            "old-jwt",
            "old-refresh",
            1_000_000_000,
        ))
        .unwrap();
        sdk
    }

    #[tokio::test]
    async fn refresh_session_replaces_the_stored_session() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(header("cookie", "refreshToken=old-refresh"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(REFRESHED_BODY, "application/json"),
            )
            .mount(&server)
            .await;
        let mut sdk = sdk_with_session(&server);

        let session = sdk.refresh_session().await.unwrap();

        assert_eq!(session.token(), "new-jwt");
        assert_eq!(session.refresh_token(), "new-refresh");
        assert_eq!(session.refresh_token_ttl(), 2_000_000_000);
    }

    #[tokio::test]
    async fn refresh_session_keeps_the_old_session_when_rejected() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_raw(r#"{"errorCode":"refresh_expired"}"#, "application/json"),
            )
            .mount(&server)
            .await;
        let mut sdk = sdk_with_session(&server);

        let error = sdk.refresh_session().await.unwrap_err();

        assert!(matches!(
            error,
            ClientError::DecodeFailed { status: 401, .. }
        ));
        assert_eq!(sdk.session().unwrap().token(), "old-jwt");
    }

    #[tokio::test]
    async fn refresh_session_without_a_session_is_an_sdk_error() {
        let mut sdk = DriveThruRpgSdk::with_config(Config::new("app-key"));

        let error = sdk.refresh_session().await.unwrap_err();

        assert!(matches!(error, ClientError::Sdk(SdkError::Unauthenticated)));
    }
}
