//! Async HTTP client for DriveThruRPG library endpoints.
//!
//! [`LibraryClient`] provides an async, authenticated interface to the DriveThruRPG API's
//! library-related endpoints, covering ordered products, download preparation, product
//! lists, and product list items.
//!
//! All methods require a valid bearer token and application key, both of which are
//! captured when the client is constructed. Create a `LibraryClient` via
//! [`DriveThruRpgSdk::library_client`] to ensure the SDK is both configured and
//! authenticated before the client is used.
//!
//! [`DriveThruRpgSdk::library_client`]: crate::DriveThruRpgSdk::library_client

use super::models::{
    LibraryItemsParams, OrderProductItemResponse, OrderProductListResponse, PageParams,
    ProductListCollectionResponse, ProductListItem, ProductListItemCreateRequest,
    ProductListItemCreateResponse, ProductListItemsResponse,
};
use crate::{config::Config, error::SdkError};

/// Maximum number of bytes logged from a failing response body.
const LOG_PAYLOAD_LIMIT: usize = 2_000;

// ── Error type ────────────────────────────────────────────────────────────────

/// Errors that can be returned by [`LibraryClient`] operations.
#[derive(Debug)]
pub enum ClientError {
    /// The SDK is not configured or not authenticated.
    ///
    /// This variant is produced when [`DriveThruRpgSdk::library_client`] is called
    /// before the SDK has been configured or before a session has been established.
    ///
    /// [`DriveThruRpgSdk::library_client`]: crate::DriveThruRpgSdk::library_client
    Sdk(SdkError),
    /// An HTTP transport or server error occurred.
    ///
    /// This wraps the underlying [`reqwest::Error`] including connection failures,
    /// timeout errors, and non-success HTTP status codes when `.error_for_status()` is
    /// used.
    Http(reqwest::Error),
    /// The provided email or password was rejected by DriveThruRPG.
    ///
    /// Returned by [`credential_login::login_with_credentials`] when
    /// `validate_login_credentials.php` indicates the credentials are invalid.
    ///
    /// [`credential_login::login_with_credentials`]: crate::auth::credential_login::login_with_credentials
    InvalidCredentials,
    /// Credentials were accepted but the application key request failed.
    ///
    /// Returned by [`credential_login::login_with_credentials`] when credentials
    /// pass validation but `create_account_app.php` returns a non-success status.
    ///
    /// [`credential_login::login_with_credentials`]: crate::auth::credential_login::login_with_credentials
    ApplicationKeyRequestFailed {
        /// The status string returned by `create_account_app.php`.
        status: String,
    },
    /// The HTTP response was received but could not be deserialized.
    ///
    /// The raw response body (truncated to [`LOG_PAYLOAD_LIMIT`] bytes) is preserved so
    /// callers can log the offending payload for diagnosis.
    DecodeFailed {
        /// The URL that was requested.
        url: String,
        /// The HTTP status code of the response.
        status: u16,
        /// The deserialization error.
        cause: serde_json::Error,
        /// Raw response body, UTF-8 lossy, truncated to [`LOG_PAYLOAD_LIMIT`] chars.
        payload: String,
    },
}

impl From<SdkError> for ClientError {
    fn from(err: SdkError) -> Self {
        ClientError::Sdk(err)
    }
}

impl From<reqwest::Error> for ClientError {
    fn from(err: reqwest::Error) -> Self {
        ClientError::Http(err)
    }
}

impl core::fmt::Display for ClientError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Sdk(err) => write!(f, "SDK error: {err}"),
            Self::Http(err) => write!(f, "HTTP error: {err}"),
            Self::InvalidCredentials => write!(f, "invalid credentials"),
            Self::ApplicationKeyRequestFailed { status } => {
                write!(f, "application key request failed (status: {status})")
            }
            Self::DecodeFailed {
                url, status, cause, ..
            } => {
                write!(f, "response decode failed [{url}] (HTTP {status}): {cause}")
            }
        }
    }
}

impl std::error::Error for ClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Sdk(err) => Some(err),
            Self::Http(err) => Some(err),
            Self::InvalidCredentials | Self::ApplicationKeyRequestFailed { .. } => None,
            Self::DecodeFailed { cause, .. } => Some(cause),
        }
    }
}

// ── LibraryClient ─────────────────────────────────────────────────────────────

/// An authenticated async HTTP client for DriveThruRPG library endpoints.
///
/// `LibraryClient` combines SDK configuration and an active bearer token to authenticate
/// all outgoing requests. Every method maps to a specific API endpoint and returns a
/// fully deserialized Rust type.
///
/// # Creating a Client
///
/// Use [`DriveThruRpgSdk::library_client`] to obtain a client that is guaranteed to have
/// both valid configuration and an active session:
///
/// ```rust,no_run
/// # use dtrpg_sdk::{Config, DriveThruRpgSdk, AuthTokenResponse};
/// # let mut sdk = DriveThruRpgSdk::with_config(Config::new("my-app-key"));
/// # sdk.apply_auth_response(AuthTokenResponse::new("token", "refresh", 9_999_999_999)).unwrap();
/// let client = sdk.library_client().unwrap();
/// ```
///
/// [`DriveThruRpgSdk::library_client`]: crate::DriveThruRpgSdk::library_client
pub struct LibraryClient {
    http: reqwest::Client,
    config: Config,
    token: String,
}

impl LibraryClient {
    /// Creates a new `LibraryClient` from the given configuration and bearer token.
    ///
    /// Prefer [`DriveThruRpgSdk::library_client`] over calling this constructor directly,
    /// as that method validates both configuration and session state before constructing
    /// the client.
    ///
    /// [`DriveThruRpgSdk::library_client`]: crate::DriveThruRpgSdk::library_client
    pub fn new(config: Config, token: String) -> Self {
        Self {
            http: reqwest::Client::new(),
            config,
            token,
        }
    }

    /// Builds the full URL for a versioned API path segment.
    ///
    /// Combines the configured base URL, API version, and the given resource path
    /// into a single URL string: `{base_url}/{api_version}/{path}`.
    fn endpoint(&self, path: &str) -> String {
        format!(
            "{}/{}/{}",
            self.config.base_url(),
            self.config.api_version(),
            path
        )
    }

    /// Returns the `Authorization` header value for the active session.
    ///
    /// The DTRPG API expects the raw JWT token without a `Bearer` prefix.
    fn auth_header(&self) -> &str {
        &self.token
    }

    /// Reads a response body and deserializes it as `T`.
    ///
    /// On failure the raw payload is logged at ERROR level (truncated to
    /// [`LOG_PAYLOAD_LIMIT`] bytes) and a [`ClientError::DecodeFailed`] is returned so
    /// callers have both the serde cause and the offending payload for diagnosis.
    async fn decode_response<T: serde::de::DeserializeOwned>(
        &self,
        url: &str,
        response: reqwest::Response,
    ) -> Result<T, ClientError> {
        let status = response.status().as_u16();
        let bytes = response.bytes().await.map_err(ClientError::Http)?;
        serde_json::from_slice::<T>(&bytes).map_err(|cause| {
            let raw = String::from_utf8_lossy(&bytes);
            let payload: String = if raw.len() > LOG_PAYLOAD_LIMIT {
                format!("{}… (truncated)", &raw[..LOG_PAYLOAD_LIMIT])
            } else {
                raw.into_owned()
            };
            tracing::error!(
                url = %url,
                status = status,
                payload = %payload,
                error = %cause,
                "API response decode failed"
            );
            ClientError::DecodeFailed {
                url: url.to_string(),
                status,
                cause,
                payload,
            }
        })
    }

    // ── Ordered Products ──────────────────────────────────────────────────────

    /// Fetches a paginated list of ordered products from the authenticated user's library.
    ///
    /// Maps to `GET /{api_version}/order_products`.
    ///
    /// Authentication is supplied via the `Authorization` header containing the raw JWT token.
    /// All fields of [`LibraryItemsParams`] that are `Some` are included as query parameters.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Http`] on any transport or deserialization failure.
    pub async fn list_order_products(
        &self,
        params: LibraryItemsParams,
    ) -> Result<OrderProductListResponse, ClientError> {
        let url = self.endpoint("order_products");

        let mut query: Vec<(&str, String)> = Vec::new();

        if let Some(page) = params.page {
            query.push(("page", page.to_string()));
        }
        if let Some(page_size) = params.page_size {
            query.push(("pageSize", page_size.to_string()));
        }
        if params.get_checksum == Some(true) {
            query.push(("getChecksum", "1".to_string()));
        }
        if params.get_filters == Some(true) {
            query.push(("getFilters", "1".to_string()));
        }
        if params.library == Some(true) {
            query.push(("library", "true".to_string()));
        }
        if let Some(archived) = params.archived {
            query.push(("archived", (if archived { "1" } else { "0" }).to_string()));
        }
        if let Some(date) = params.updated_date_after {
            query.push(("updatedDate[after]", date));
        }

        tracing::debug!(method = "GET", url = %url, "SDK request");
        let response = self
            .http
            .get(&url)
            .query(&query)
            .header("Authorization", self.auth_header())
            .send()
            .await?;
        tracing::debug!(url = %url, status = response.status().as_u16(), "SDK response");

        self.decode_response(&url, response).await
    }

    /// Fetches the details of a single ordered product by its identifier.
    ///
    /// Maps to `GET /{api_version}/order_products/{order_product_id}`.
    ///
    /// Authentication is supplied via the `Authorization` header containing the raw JWT token.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Http`] on any transport or deserialization failure.
    pub async fn get_order_product(
        &self,
        order_product_id: u64,
    ) -> Result<OrderProductItemResponse, ClientError> {
        let url = self.endpoint(&format!("order_products/{order_product_id}"));

        tracing::debug!(method = "GET", url = %url, "SDK request");
        let response = self
            .http
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;
        tracing::debug!(url = %url, status = response.status().as_u16(), "SDK response");

        self.decode_response(&url, response).await
    }

    /// Prepares a download for the given ordered product and returns the raw API response.
    ///
    /// Maps to `GET /{api_version}/order_products/{order_product_id}/prepare`.
    ///
    /// The response is returned as a [`serde_json::Value`] because the schema for this
    /// endpoint has not yet been formally defined by the API contract. The type will be
    /// tightened in a future change once the API contract matures.
    ///
    /// Authentication is supplied via the `Authorization` header containing the raw JWT token.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Http`] on any transport or deserialization failure.
    pub async fn prepare_download(
        &self,
        order_product_id: u64,
    ) -> Result<serde_json::Value, ClientError> {
        let url = self.endpoint(&format!("order_products/{order_product_id}/prepare"));

        tracing::debug!(method = "GET", url = %url, "SDK request");
        let response = self
            .http
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;
        tracing::debug!(url = %url, status = response.status().as_u16(), "SDK response");

        self.decode_response(&url, response).await
    }

    // ── Product Lists ─────────────────────────────────────────────────────────

    /// Fetches a paginated list of product lists belonging to the authenticated user.
    ///
    /// Maps to `GET /{api_version}/product_lists`.
    ///
    /// Authentication is supplied via the `Authorization` header containing the raw JWT token.
    /// Pagination is controlled via [`PageParams`].
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Http`] on any transport or deserialization failure.
    pub async fn list_product_lists(
        &self,
        params: PageParams,
    ) -> Result<ProductListCollectionResponse, ClientError> {
        let url = self.endpoint("product_lists");

        let mut query: Vec<(&str, String)> = Vec::new();

        if let Some(page) = params.page {
            query.push(("page", page.to_string()));
        }
        if let Some(page_size) = params.page_size {
            query.push(("pageSize", page_size.to_string()));
        }

        tracing::debug!(method = "GET", url = %url, "SDK request");
        let response = self
            .http
            .get(&url)
            .query(&query)
            .header("Authorization", self.auth_header())
            .send()
            .await?;
        tracing::debug!(url = %url, status = response.status().as_u16(), "SDK response");

        self.decode_response(&url, response).await
    }

    /// Fetches a paginated list of items within a specific product list.
    ///
    /// Maps to `GET /{api_version}/product_list_items?productListId={product_list_id}`.
    ///
    /// Authentication is supplied via the `Authorization` header containing the raw JWT token.
    /// Pagination is controlled via [`PageParams`].
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Http`] on any transport or deserialization failure.
    pub async fn list_product_list_items(
        &self,
        product_list_id: u64,
        params: PageParams,
    ) -> Result<ProductListItemsResponse, ClientError> {
        let url = self.endpoint("product_list_items");

        let mut query: Vec<(&str, String)> = vec![("productListId", product_list_id.to_string())];

        if let Some(page) = params.page {
            query.push(("page", page.to_string()));
        }
        if let Some(page_size) = params.page_size {
            query.push(("pageSize", page_size.to_string()));
        }

        tracing::debug!(method = "GET", url = %url, "SDK request");
        let response = self
            .http
            .get(&url)
            .query(&query)
            .header("Authorization", self.auth_header())
            .send()
            .await?;
        tracing::debug!(url = %url, status = response.status().as_u16(), "SDK response");

        self.decode_response(&url, response).await
    }

    /// Creates a new product list with the given name.
    ///
    /// Maps to `POST /{api_version}/product_lists` with a JSON body `{"name": "<name>"}`.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Http`] on transport failure or [`ClientError::DecodeFailed`]
    /// if the response cannot be deserialized.
    pub async fn create_product_list(&self, name: &str) -> Result<ProductListItem, ClientError> {
        let url = self.endpoint("product_lists");

        tracing::debug!(method = "POST", url = %url, "SDK request");
        let response = self
            .http
            .post(&url)
            .header("Authorization", self.auth_header())
            .json(&serde_json::json!({ "name": name }))
            .send()
            .await?;
        tracing::debug!(url = %url, status = response.status().as_u16(), "SDK response");

        self.decode_response(&url, response).await
    }

    /// Deletes a product list by id.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError`] if the request fails.
    pub async fn delete_product_list(&self, id: u64) -> Result<(), ClientError> {
        let url = self.endpoint(&format!("product_lists/{id}"));

        tracing::debug!(method = "DELETE", url = %url, "SDK request");
        let response = self
            .http
            .delete(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;
        tracing::debug!(url = %url, status = response.status().as_u16(), "SDK response");

        response
            .error_for_status()
            .map(|_| ())
            .map_err(ClientError::Http)
    }

    /// Adds a product to a product list as a member.
    ///
    /// Maps to `POST /{api_version}/product_list_items`.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Http`] on transport failure or [`ClientError::DecodeFailed`]
    /// if the response cannot be deserialized.
    pub async fn add_product_list_item(
        &self,
        product_list_id: u64,
        product_id: u64,
    ) -> Result<ProductListItemCreateResponse, ClientError> {
        let url = self.endpoint("product_list_items");
        let body = ProductListItemCreateRequest {
            product_id,
            product_list_id,
        };

        tracing::debug!(method = "POST", url = %url, "SDK request");
        let response = self
            .http
            .post(&url)
            .header("Authorization", self.auth_header())
            .json(&body)
            .send()
            .await?;
        tracing::debug!(url = %url, status = response.status().as_u16(), "SDK response");

        self.decode_response(&url, response).await
    }

    /// Removes a product list item by its own id (not the product's id).
    ///
    /// Maps to `DELETE /{api_version}/product_list_items/{product_list_item_id}`.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError`] if the request fails.
    pub async fn delete_product_list_item(
        &self,
        product_list_item_id: u64,
    ) -> Result<(), ClientError> {
        let url = self.endpoint(&format!("product_list_items/{product_list_item_id}"));

        tracing::debug!(method = "DELETE", url = %url, "SDK request");
        let response = self
            .http
            .delete(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await?;
        tracing::debug!(url = %url, status = response.status().as_u16(), "SDK response");

        response
            .error_for_status()
            .map(|_| ())
            .map_err(ClientError::Http)
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::*;

    fn client_for(server: &MockServer) -> LibraryClient {
        let config = Config::with_base_url("test-app-key", server.uri());
        LibraryClient::new(config, "test-token".to_string())
    }

    #[tokio::test]
    async fn add_product_list_item_returns_created_item() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/vBeta/product_list_items"))
            .and(body_json(serde_json::json!({
                "productId": 515_276,
                "productListId": 86_151,
            })))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "productId": 515_276,
                "productListId": 86_151,
                "productListItemId": 2_629_321,
            })))
            .expect(1)
            .mount(&server)
            .await;

        let client = client_for(&server);
        let result = client.add_product_list_item(86_151, 515_276).await.unwrap();

        assert_eq!(result.product_id, 515_276);
        assert_eq!(result.product_list_id, 86_151);
        assert_eq!(result.product_list_item_id, 2_629_321);
    }

    #[tokio::test]
    async fn add_product_list_item_returns_http_error_on_failure_status() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/vBeta/product_list_items"))
            .respond_with(ResponseTemplate::new(404))
            .expect(1)
            .mount(&server)
            .await;

        let client = client_for(&server);
        let result = client.add_product_list_item(86_151, 515_276).await;

        assert!(matches!(
            result,
            Err(ClientError::DecodeFailed { status: 404, .. })
        ));
    }

    #[tokio::test]
    async fn delete_product_list_item_succeeds_on_no_content() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/vBeta/product_list_items/2629321"))
            .respond_with(ResponseTemplate::new(204))
            .expect(1)
            .mount(&server)
            .await;

        let client = client_for(&server);
        let result = client.delete_product_list_item(2_629_321).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn delete_product_list_item_returns_http_error_on_failure_status() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/vBeta/product_list_items/2629321"))
            .respond_with(ResponseTemplate::new(404))
            .expect(1)
            .mount(&server)
            .await;

        let client = client_for(&server);
        let result = client.delete_product_list_item(2_629_321).await;

        assert!(matches!(result, Err(ClientError::Http(_))));
    }
}
