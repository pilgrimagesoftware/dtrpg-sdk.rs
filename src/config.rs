#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    application_key: String,
    base_url: String,
}

impl Config {
    pub const DEFAULT_BASE_URL: &str = "https://api.drivethrurpg.com/api";

    pub fn new(application_key: impl Into<String>) -> Self {
        Self {
            application_key: application_key.into(),
            base_url: Self::DEFAULT_BASE_URL.to_string(),
        }
    }

    pub fn with_base_url(application_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            application_key: application_key.into(),
            base_url: base_url.into(),
        }
    }

    pub fn application_key(&self) -> &str {
        &self.application_key
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}
