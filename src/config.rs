//! SDK configuration types.
//!
//! [`Config`] holds the application-level settings required to make requests to the
//! DriveThruRPG API: the publisher/application key and the API base URL.

/// Configuration for the DriveThruRPG SDK.
///
/// A `Config` must be provided to [`DriveThruRpgSdk`] before any authenticated API
/// calls can be made. It binds an application key to an API endpoint, defaulting to
/// the production DriveThruRPG API.
///
/// # Examples
///
/// ```rust
/// use dtrpg_sdk::Config;
///
/// // Use the production API with your application key.
/// let config = Config::new("my-app-key");
///
/// // Point at a local test server instead.
/// let staging = Config::with_base_url("my-app-key", "http://localhost:8080/api");
/// ```
///
/// [`DriveThruRpgSdk`]: crate::DriveThruRpgSdk
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    application_key: String,
    base_url: String,
}

impl Config {
    /// The production DriveThruRPG API base URL used when no custom URL is provided.
    pub const DEFAULT_BASE_URL: &str = "https://api.drivethrurpg.com/api";

    /// Creates a `Config` targeting the production API with the given `application_key`.
    pub fn new(application_key: impl Into<String>) -> Self {
        Self {
            application_key: application_key.into(),
            base_url: Self::DEFAULT_BASE_URL.to_string(),
        }
    }

    /// Creates a `Config` with a custom `base_url`, overriding the production endpoint.
    ///
    /// Useful for pointing the SDK at a staging server or a local mock during development.
    pub fn with_base_url(application_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            application_key: application_key.into(),
            base_url: base_url.into(),
        }
    }

    /// Returns the application key used to identify this client to the API.
    pub fn application_key(&self) -> &str {
        &self.application_key
    }

    /// Returns the base URL that SDK requests will be sent to.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}
