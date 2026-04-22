use core::fmt;

use crate::error::AuthSessionError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthTokenResponse {
    pub token: String,
    pub refresh_token: String,
    pub refresh_token_ttl: u64,
}

impl AuthTokenResponse {
    pub fn new(
        token: impl Into<String>,
        refresh_token: impl Into<String>,
        refresh_token_ttl: u64,
    ) -> Self {
        Self {
            token: token.into(),
            refresh_token: refresh_token.into(),
            refresh_token_ttl,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthState {
    Unauthenticated,
    TokenInvalid,
    TokenExpired,
    RefreshExpired,
    Unauthorized,
}

impl AuthState {
    pub fn as_api_str(&self) -> &'static str {
        match self {
            Self::Unauthenticated => "unauthenticated",
            Self::TokenInvalid => "token_invalid",
            Self::TokenExpired => "token_expired",
            Self::RefreshExpired => "refresh_expired",
            Self::Unauthorized => "unauthorized",
        }
    }
}

impl fmt::Display for AuthState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_api_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthSession {
    token: String,
    refresh_token: String,
    refresh_token_ttl: u64,
}

impl AuthSession {
    pub fn from_api_response(response: AuthTokenResponse) -> Self {
        Self {
            token: response.token,
            refresh_token: response.refresh_token,
            refresh_token_ttl: response.refresh_token_ttl,
        }
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn refresh_token(&self) -> &str {
        &self.refresh_token
    }

    pub fn refresh_token_ttl(&self) -> u64 {
        self.refresh_token_ttl
    }

    pub fn refresh_token_expired_at(&self, unix_timestamp: u64) -> bool {
        unix_timestamp >= self.refresh_token_ttl
    }

    pub fn invalidate(self, error: AuthSessionError) -> SessionTransition {
        SessionTransition {
            next_session: None,
            error,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionTransition {
    pub next_session: Option<AuthSession>,
    pub error: AuthSessionError,
}
