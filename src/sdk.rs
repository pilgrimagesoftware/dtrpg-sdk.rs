use crate::{
    auth::{AuthSession, AuthTokenResponse},
    config::Config,
    error::{AuthSessionError, SdkError},
};

#[derive(Clone, Debug, Default)]
pub struct DriveThruRpgSdk {
    config: Option<Config>,
    session: Option<AuthSession>,
}

impl DriveThruRpgSdk {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_config(config: Config) -> Self {
        Self {
            config: Some(config),
            session: None,
        }
    }

    pub fn configure(&mut self, config: Config) {
        self.config = Some(config);
    }

    pub fn config(&self) -> Option<&Config> {
        self.config.as_ref()
    }

    pub fn session(&self) -> Option<&AuthSession> {
        self.session.as_ref()
    }

    pub fn require_config(&self) -> Result<&Config, SdkError> {
        self.config.as_ref().ok_or(SdkError::Unconfigured)
    }

    pub fn require_session(&self) -> Result<&AuthSession, SdkError> {
        self.session.as_ref().ok_or(SdkError::Unauthenticated)
    }

    pub fn apply_auth_response(
        &mut self,
        response: AuthTokenResponse,
    ) -> Result<&AuthSession, SdkError> {
        self.require_config()?;
        self.session = Some(AuthSession::from_api_response(response));
        self.require_session()
    }

    pub fn clear_session(&mut self) {
        self.session = None;
    }

    pub fn invalidate_session(
        &mut self,
        error: AuthSessionError,
    ) -> Result<AuthSessionError, SdkError> {
        self.require_session()?;
        self.clear_session();
        Ok(error)
    }
}
