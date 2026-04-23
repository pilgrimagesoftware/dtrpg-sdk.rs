//! Authentication types for the DriveThruRPG SDK.
//!
//! This module provides the core types for representing and managing the authentication
//! lifecycle with the DriveThruRPG API:
//!
//! - [`AuthTokenResponse`] — the raw token payload received from the API after a successful login.
//! - [`AuthState`] — an enumeration of authentication failure states reported by the API.
//! - [`AuthSession`] — an active, validated session derived from an [`AuthTokenResponse`].
//! - [`SessionTransition`] — the result of invalidating a session, carrying the error that caused
//!   the transition and an optional replacement session.

use core::fmt;

use crate::error::AuthSessionError;

/// The raw authentication token payload returned by the DriveThruRPG API.
///
/// This struct is a direct representation of the API response fields. Callers should
/// convert it into an [`AuthSession`] via [`AuthSession::from_api_response`] before
/// treating the session as active.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthTokenResponse {
    /// The short-lived JWT access token used to authenticate API requests.
    pub token: String,
    /// The long-lived refresh token used to obtain a new access token.
    pub refresh_token: String,
    /// Unix timestamp (seconds) at which the refresh token expires.
    pub refresh_token_ttl: u64,
}

impl AuthTokenResponse {
    /// Constructs a new `AuthTokenResponse` from the given token fields.
    ///
    /// `refresh_token_ttl` is a Unix timestamp (seconds since the epoch) representing
    /// the absolute expiry of the refresh token.
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

/// The authentication failure state reported by the DriveThruRPG API.
///
/// Each variant maps to a specific string value in the API protocol (see [`AuthState::as_api_str`]).
/// Use this to understand *why* a session was invalidated and decide how to recover.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AuthState {
    /// No authentication credentials are present.
    Unauthenticated,
    /// The provided access token is structurally invalid or unrecognized.
    TokenInvalid,
    /// The access token has passed its expiry time and must be refreshed.
    TokenExpired,
    /// The refresh token has passed its expiry time; the user must re-authenticate.
    RefreshExpired,
    /// The credentials are valid but the caller lacks permission for the requested resource.
    Unauthorized,
}

impl AuthState {
    /// Returns the API wire string for this state, suitable for comparison with API error codes.
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

/// An active authentication session with the DriveThruRPG API.
///
/// `AuthSession` is the validated, runtime representation of an authenticated user.
/// It is obtained by calling [`DriveThruRpgSdk::apply_auth_response`] with a token
/// response from the API.
///
/// [`DriveThruRpgSdk::apply_auth_response`]: crate::DriveThruRpgSdk::apply_auth_response
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthSession {
    token: String,
    refresh_token: String,
    refresh_token_ttl: u64,
}

impl AuthSession {
    /// Creates an `AuthSession` from a raw [`AuthTokenResponse`].
    pub fn from_api_response(response: AuthTokenResponse) -> Self {
        Self {
            token: response.token,
            refresh_token: response.refresh_token,
            refresh_token_ttl: response.refresh_token_ttl,
        }
    }

    /// Returns the short-lived JWT access token for this session.
    pub fn token(&self) -> &str {
        &self.token
    }

    /// Returns the long-lived refresh token for this session.
    pub fn refresh_token(&self) -> &str {
        &self.refresh_token
    }

    /// Returns the Unix timestamp (seconds) at which the refresh token expires.
    pub fn refresh_token_ttl(&self) -> u64 {
        self.refresh_token_ttl
    }

    /// Returns `true` if `unix_timestamp` is at or past the refresh token's expiry.
    ///
    /// Use the current wall-clock time (as a Unix timestamp) to determine whether
    /// the session's refresh token is still usable.
    pub fn refresh_token_expired_at(&self, unix_timestamp: u64) -> bool {
        unix_timestamp >= self.refresh_token_ttl
    }

    /// Consumes the session and produces a [`SessionTransition`] representing its invalidation.
    ///
    /// The resulting transition has no replacement session (`next_session` is `None`) and
    /// carries the provided `error` describing why the session was invalidated.
    pub fn invalidate(self, error: AuthSessionError) -> SessionTransition {
        SessionTransition {
            next_session: None,
            error,
        }
    }
}

/// The outcome of invalidating an [`AuthSession`].
///
/// A `SessionTransition` is returned when a session ends due to an error. It records
/// the error that caused the invalidation and an optional replacement session (for
/// cases where token refresh succeeds mid-invalidation).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionTransition {
    /// The replacement session, if one was established as part of this transition.
    ///
    /// This is `None` when the session was simply terminated without a refresh.
    pub next_session: Option<AuthSession>,
    /// The error that caused this session transition.
    pub error: AuthSessionError,
}
