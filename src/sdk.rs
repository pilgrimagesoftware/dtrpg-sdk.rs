//! The primary SDK entry point.
//!
//! [`DriveThruRpgSdk`] is the root object that holds SDK configuration and manages the
//! active authentication session. Start here when integrating the DriveThruRPG API into
//! your application.

use crate::{
    auth::{AuthSession, AuthTokenResponse},
    config::Config,
    error::{AuthSessionError, SdkError},
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
