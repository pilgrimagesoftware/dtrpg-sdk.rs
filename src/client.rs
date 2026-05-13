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

use crate::{
    config::Config,
    error::SdkError,
    library::{
        LibraryItemsParams, OrderProductItemResponse, OrderProductListResponse, PageParams,
        ProductListCollectionResponse, ProductListItemsResponse,
    },
};

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
        }
    }
}

impl std::error::Error for ClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Sdk(err) => Some(err),
            Self::Http(err) => Some(err),
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

    /// Returns the `Authorization: Bearer <token>` header value for the active session.
    fn auth_header(&self) -> String {
        format!("Bearer {}", self.token)
    }

    // ── Ordered Products ──────────────────────────────────────────────────────

    /// Fetches a paginated list of ordered products from the authenticated user's library.
    ///
    /// Maps to `GET /{api_version}/order_products`.
    ///
    /// Authentication is supplied via both the `applicationKey` query parameter and the
    /// `Authorization: Bearer` header as required by the API contract. All fields of
    /// [`LibraryItemsParams`] that are `Some` are included as additional query parameters.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Http`] on any transport or deserialization failure.
    pub async fn list_order_products(
        &self,
        params: LibraryItemsParams,
    ) -> Result<OrderProductListResponse, ClientError> {
        let url = self.endpoint("order_products");

        let mut query: Vec<(&str, String)> =
            vec![("applicationKey", self.config.application_key().to_string())];

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

        let response = self
            .http
            .get(&url)
            .query(&query)
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .json::<OrderProductListResponse>()
            .await?;

        Ok(response)
    }

    /// Fetches the details of a single ordered product by its identifier.
    ///
    /// Maps to `GET /{api_version}/order_products/{order_product_id}`.
    ///
    /// Authentication is supplied via both the `applicationKey` query parameter and the
    /// `Authorization: Bearer` header.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Http`] on any transport or deserialization failure.
    pub async fn get_order_product(
        &self,
        order_product_id: u64,
    ) -> Result<OrderProductItemResponse, ClientError> {
        let url = self.endpoint(&format!("order_products/{order_product_id}"));

        let query = [("applicationKey", self.config.application_key().to_string())];

        let response = self
            .http
            .get(&url)
            .query(&query)
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .json::<OrderProductItemResponse>()
            .await?;

        Ok(response)
    }

    /// Prepares a download for the given ordered product and returns the raw API response.
    ///
    /// Maps to `GET /{api_version}/order_products/{order_product_id}/prepare`.
    ///
    /// The response is returned as a [`serde_json::Value`] because the schema for this
    /// endpoint has not yet been formally defined by the API contract. The type will be
    /// tightened in a future change once the API contract matures.
    ///
    /// Authentication is supplied via both the `applicationKey` query parameter and the
    /// `Authorization: Bearer` header.
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Http`] on any transport or deserialization failure.
    pub async fn prepare_download(
        &self,
        order_product_id: u64,
    ) -> Result<serde_json::Value, ClientError> {
        let url = self.endpoint(&format!("order_products/{order_product_id}/prepare"));

        let query = [("applicationKey", self.config.application_key().to_string())];

        let response = self
            .http
            .get(&url)
            .query(&query)
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(response)
    }

    // ── Product Lists ─────────────────────────────────────────────────────────

    /// Fetches a paginated list of product lists belonging to the authenticated user.
    ///
    /// Maps to `GET /{api_version}/product_lists`.
    ///
    /// Authentication is supplied via both the `applicationKey` query parameter and the
    /// `Authorization: Bearer` header. Pagination is controlled via [`PageParams`].
    ///
    /// # Errors
    ///
    /// Returns [`ClientError::Http`] on any transport or deserialization failure.
    pub async fn list_product_lists(
        &self,
        params: PageParams,
    ) -> Result<ProductListCollectionResponse, ClientError> {
        let url = self.endpoint("product_lists");

        let mut query: Vec<(&str, String)> =
            vec![("applicationKey", self.config.application_key().to_string())];

        if let Some(page) = params.page {
            query.push(("page", page.to_string()));
        }
        if let Some(page_size) = params.page_size {
            query.push(("pageSize", page_size.to_string()));
        }

        let response = self
            .http
            .get(&url)
            .query(&query)
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .json::<ProductListCollectionResponse>()
            .await?;

        Ok(response)
    }

    /// Fetches a paginated list of items within a specific product list.
    ///
    /// Maps to `GET /{api_version}/product_list_items?productListId={product_list_id}`.
    ///
    /// Authentication is supplied via both the `applicationKey` query parameter and the
    /// `Authorization: Bearer` header. Pagination is controlled via [`PageParams`].
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

        let mut query: Vec<(&str, String)> = vec![
            ("applicationKey", self.config.application_key().to_string()),
            ("productListId", product_list_id.to_string()),
        ];

        if let Some(page) = params.page {
            query.push(("page", page.to_string()));
        }
        if let Some(page_size) = params.page_size {
            query.push(("pageSize", page_size.to_string()));
        }

        let response = self
            .http
            .get(&url)
            .query(&query)
            .header("Authorization", self.auth_header())
            .send()
            .await?
            .json::<ProductListItemsResponse>()
            .await?;

        Ok(response)
    }
}
