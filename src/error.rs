//! Error types for the DriveThruRPG SDK.
//!
//! This module defines two error types:
//!
//! - [`SdkError`] — top-level errors covering SDK lifecycle states such as missing
//!   configuration or an unauthenticated session.
//! - [`AuthSessionError`] — structured errors returned by the API when an authentication
//!   session is rejected or invalidated.

use core::fmt;

use crate::auth::AuthState;

/// A top-level error returned by SDK operations.
///
/// Most SDK methods return `Result<_, SdkError>`. Check this type to determine
/// whether the failure is a configuration issue, an auth state issue, or a
/// structured API-level session error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SdkError {
    /// The SDK has not been configured with a [`Config`] yet.
    ///
    /// Call [`DriveThruRpgSdk::configure`] or construct the SDK with
    /// [`DriveThruRpgSdk::with_config`] before making API calls.
    ///
    /// [`Config`]: crate::Config
    /// [`DriveThruRpgSdk::configure`]: crate::DriveThruRpgSdk::configure
    /// [`DriveThruRpgSdk::with_config`]: crate::DriveThruRpgSdk::with_config
    Unconfigured,
    /// The SDK has no active authentication session.
    ///
    /// Obtain a session by calling [`DriveThruRpgSdk::apply_auth_response`] with a
    /// successful token response from the API.
    ///
    /// [`DriveThruRpgSdk::apply_auth_response`]: crate::DriveThruRpgSdk::apply_auth_response
    Unauthenticated,
    /// The current session was invalidated with a structured API error.
    AuthSession(AuthSessionError),
}

impl fmt::Display for SdkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unconfigured => write!(f, "SDK is not configured"),
            Self::Unauthenticated => write!(f, "SDK does not have an authenticated session"),
            Self::AuthSession(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for SdkError {}

/// A structured authentication error returned by the DriveThruRPG API.
///
/// This type carries the machine-readable `error_code`, a human-readable `message`,
/// and an [`AuthState`] that classifies the failure. It is used when the API explicitly
/// rejects an operation due to an auth-related condition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthSessionError {
    /// The machine-readable error code from the API (e.g. `"token_expired"`).
    pub error_code: String,
    /// A human-readable description of the error.
    pub message: String,
    /// The [`AuthState`] classification for this error.
    pub auth_state: AuthState,
}

impl AuthSessionError {
    /// Constructs a new `AuthSessionError` from its components.
    pub fn new(
        error_code: impl Into<String>,
        message: impl Into<String>,
        auth_state: AuthState,
    ) -> Self {
        Self {
            error_code: error_code.into(),
            message: message.into(),
            auth_state,
        }
    }
}

impl fmt::Display for AuthSessionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}) [{}]",
            self.message, self.error_code, self.auth_state
        )
    }
}

impl std::error::Error for AuthSessionError {}
