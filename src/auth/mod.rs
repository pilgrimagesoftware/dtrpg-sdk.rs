//! Authentication: session types, application-key exchange, and website credential login.
//!
//! [`AuthSession`], [`AuthState`], [`AuthTokenResponse`], and [`SessionTransition`] are the
//! session/state types shared across the SDK. [`key_exchange`] exchanges an application key
//! for a session token against `api.drivethrurpg.com`. [`credential_login`] exchanges an
//! email/password pair for an application key against `www.drivethrurpg.com`, upstream of
//! [`key_exchange::authenticate`].

pub mod credential_login;
pub mod key_exchange;
mod session;

pub use session::{AuthSession, AuthState, AuthTokenResponse, SessionTransition};
