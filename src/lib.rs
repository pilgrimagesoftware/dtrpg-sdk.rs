pub mod auth;
pub mod config;
pub mod error;
pub mod sdk;

pub use auth::{AuthSession, AuthState, AuthTokenResponse, SessionTransition};
pub use config::Config;
pub use error::{AuthSessionError, SdkError};
pub use sdk::DriveThruRpgSdk;

#[cfg(test)]
mod tests {
    use super::{
        AuthSessionError, AuthState, AuthTokenResponse, Config, DriveThruRpgSdk, SdkError,
    };

    #[test]
    fn sdk_requires_configuration_before_auth_session_is_applied() {
        let mut sdk = DriveThruRpgSdk::new();
        let response = AuthTokenResponse::new("jwt-token", "refresh-token", 1_771_547_233);

        let error = sdk.apply_auth_response(response).unwrap_err();

        assert_eq!(error, SdkError::Unconfigured);
    }

    #[test]
    fn sdk_stores_api_defined_auth_session_after_configuration() {
        let mut sdk = DriveThruRpgSdk::with_config(Config::new("app-key"));
        let response = AuthTokenResponse::new("jwt-token", "refresh-token", 1_771_547_233);

        let session = sdk.apply_auth_response(response).unwrap();

        assert_eq!(session.token(), "jwt-token");
        assert_eq!(session.refresh_token(), "refresh-token");
        assert!(!session.refresh_token_expired_at(1_771_547_232));
        assert!(session.refresh_token_expired_at(1_771_547_233));
    }

    #[test]
    fn invalidating_session_preserves_api_auth_state_meaning() {
        let mut sdk = DriveThruRpgSdk::with_config(Config::new("app-key"));
        let response = AuthTokenResponse::new("jwt-token", "refresh-token", 1_771_547_233);
        sdk.apply_auth_response(response).unwrap();

        let error = AuthSessionError::new(
            "token_expired",
            "The authentication token has expired.",
            AuthState::TokenExpired,
        );

        let invalidation = sdk.invalidate_session(error.clone()).unwrap();

        assert_eq!(invalidation, error);
        assert_eq!(
            sdk.require_session().unwrap_err(),
            SdkError::Unauthenticated
        );
    }
}
