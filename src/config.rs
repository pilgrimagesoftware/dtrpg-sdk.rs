//! SDK configuration types.
//!
//! [`Config`] holds the application-level settings required to make requests to the
//! DriveThruRPG API: the publisher/application key, the API base URL, and the API version
//! segment used in request URLs.

/// Configuration for the DriveThruRPG SDK.
///
/// A `Config` must be provided to [`DriveThruRpgSdk`] before any authenticated API
/// calls can be made. It binds an application key to an API endpoint, defaulting to
/// the production DriveThruRPG API and the current `"vBeta"` API version.
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
    api_version: String,
}

impl Config {
    /// The production DriveThruRPG API base URL used when no custom URL is provided.
    pub const DEFAULT_BASE_URL: &str = "https://api.drivethrurpg.com/api";

    /// The API version path segment used when no custom version is provided.
    pub const DEFAULT_API_VERSION: &str = "vBeta";

    /// Creates a `Config` targeting the production API with the given `application_key`.
    ///
    /// The base URL defaults to [`DEFAULT_BASE_URL`] and the API version defaults to
    /// [`DEFAULT_API_VERSION`].
    ///
    /// [`DEFAULT_BASE_URL`]: Config::DEFAULT_BASE_URL
    /// [`DEFAULT_API_VERSION`]: Config::DEFAULT_API_VERSION
    pub fn new(application_key: impl Into<String>) -> Self {
        Self {
            application_key: application_key.into(),
            base_url: Self::DEFAULT_BASE_URL.to_string(),
            api_version: Self::DEFAULT_API_VERSION.to_string(),
        }
    }

    /// Creates a `Config` with a custom `base_url`, overriding the production endpoint.
    ///
    /// The API version defaults to [`DEFAULT_API_VERSION`]. Useful for pointing the SDK
    /// at a staging server or a local mock during development.
    ///
    /// [`DEFAULT_API_VERSION`]: Config::DEFAULT_API_VERSION
    pub fn with_base_url(application_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            application_key: application_key.into(),
            base_url: base_url.into(),
            api_version: Self::DEFAULT_API_VERSION.to_string(),
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

    /// Returns the API version path segment used when constructing endpoint URLs.
    ///
    /// This value is inserted between the base URL and the resource path:
    /// `{base_url}/{api_version}/{resource}`. Defaults to `"vBeta"`.
    pub fn api_version(&self) -> &str {
        &self.api_version
    }
}
