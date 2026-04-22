use core::fmt;

use crate::auth::AuthState;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SdkError {
    Unconfigured,
    Unauthenticated,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthSessionError {
    pub error_code: String,
    pub message: String,
    pub auth_state: AuthState,
}

impl AuthSessionError {
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
