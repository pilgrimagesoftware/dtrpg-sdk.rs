//! Library resource types for the DriveThruRPG SDK.
//!
//! This module provides Rust model types that mirror the API-defined schemas for library
//! resources: ordered products, product files, product lists, and associated pagination
//! and metadata structures.
//!
//! All response types implement [`serde::Deserialize`] (and [`serde::Serialize`]) so they
//! can be decoded directly from JSON returned by the DriveThruRPG API. Field names follow
//! the API contract and are mapped from camelCase JSON to snake_case Rust conventions via
//! `#[serde(rename = "...")]` attributes.
//!
//! Query parameter structs ([`LibraryItemsParams`], [`PageParams`]) are plain Rust structs
//! with no serde requirement; they are consumed by [`LibraryClient`] methods to build
//! URL query strings.
//!
//! [`LibraryClient`]: crate::LibraryClient

use serde::{Deserialize, Deserializer, Serialize};

/// Deserializes a JSON null or missing field as the type's default value.
///
/// Use via `#[serde(default, deserialize_with = "null_as_default")]` on fields that the
/// API may send as `null` but that should be treated as empty collections or zero values.
fn null_as_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}

// ── Pagination ────────────────────────────────────────────────────────────────

/// Pagination links included in all paginated API responses.
///
/// The `self_` field corresponds to the JSON key `"self"`, which is a Rust reserved
/// keyword and is therefore renamed at the struct level via `#[serde(rename = "self")]`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaginationLinks {
    /// The canonical URL for the current page of results.
    #[serde(rename = "self")]
    pub self_: String,
    /// URL for the first page of results, if available.
    pub first: Option<String>,
    /// URL for the last page of results, if available.
    pub last: Option<String>,
    /// URL for the previous page of results, if available.
    pub prev: Option<String>,
    /// URL for the next page of results, if available.
    pub next: Option<String>,
}

/// Pagination metadata included in all paginated API responses.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaginationMeta {
    /// The number of items returned per page.
    #[serde(rename = "itemsPerPage")]
    pub items_per_page: u32,
    /// The current page number (1-based).
    #[serde(rename = "currentPage")]
    pub current_page: u32,
}

// ── File / Checksum ───────────────────────────────────────────────────────────

/// Checksum information for a single downloadable product file.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileChecksum {
    /// The checksum hash string for the file.
    pub checksum: String,
    /// The date when the checksum was generated (ISO 8601 string).
    #[serde(rename = "checksumDate")]
    pub checksum_date: String,
}

/// A downloadable file associated with an ordered product.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderProductFile {
    /// The index of this file within the ordered product's file list.
    pub index: u32,
    /// The unique identifier for this specific download record.
    #[serde(rename = "orderProductDownloadId")]
    pub order_product_download_id: u64,
    /// The display title of the file.
    pub title: String,
    /// The filename as it will appear when downloaded.
    pub filename: String,
    /// The file size in bytes.
    pub size: u64,
    /// The file size expressed in megabytes as a formatted string.
    #[serde(rename = "sizeMB")]
    pub size_mb: String,
    /// Checksums available for verifying the integrity of the downloaded file.
    ///
    /// The API may return `null` for products without checksum data; treated as empty.
    #[serde(default, deserialize_with = "null_as_default")]
    pub checksums: Vec<FileChecksum>,
}

// ── Filters / History / Attributes ───────────────────────────────────────────

/// A filter category associated with an ordered product.
///
/// Populated when `getFilters=1` is included in the request.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderProductFilter {
    /// The unique identifier of this filter category.
    #[serde(rename = "filterId")]
    pub filter_id: u64,
    /// The unique identifier of this filter's parent category.
    #[serde(rename = "parentFilterId")]
    pub parent_filter_id: u64,
    /// The display name of this filter category.
    pub name: String,
    /// The display name of this filter's parent category.
    #[serde(rename = "parentName")]
    pub parent_name: String,
}

/// A single history entry recording a change made to an ordered product.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderProductHistoryEntry {
    /// The date and time when the change occurred (ISO 8601 string).
    pub changed: String,
    /// A human-readable description of what changed.
    pub changes: String,
}

/// An individual attribute option associated with an ordered product.
///
/// Attributes describe purchase options such as format or edition.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderProductAttribute {
    /// The unique identifier of the order this attribute belongs to.
    #[serde(rename = "orderId")]
    pub order_id: u64,
    /// The name of the option (e.g., `"Format"`).
    #[serde(rename = "optionName")]
    pub option_name: String,
    /// The display name of the selected option value (e.g., `"PDF"`).
    #[serde(rename = "optionValueName")]
    pub option_value_name: String,
    /// The price associated with this option, as a formatted string.
    pub price: String,
    /// A prefix to display before the price (e.g., `"$"`).
    #[serde(rename = "pricePrefix")]
    pub price_prefix: String,
    /// The unique identifier for the selected option value.
    #[serde(rename = "optionValueId")]
    pub option_value_id: u64,
    /// The type classification of this option.
    #[serde(rename = "optionType")]
    pub option_type: String,
}

// ── OrderProduct ──────────────────────────────────────────────────────────────

/// The full attribute set for an ordered product.
///
/// This is the primary payload within an [`OrderProductItem`]. It includes required fields
/// present on every ordered product as well as optional collections (filters, history,
/// attributes) that are populated only when specifically requested.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderProductAttributes {
    /// The unique identifier of the order this product belongs to.
    #[serde(rename = "orderId")]
    pub order_id: u64,
    /// The unique identifier of the product.
    #[serde(rename = "productId")]
    pub product_id: u64,
    /// The publisher identifier used for royalty tracking.
    #[serde(rename = "royaltyPublisherId")]
    pub royalty_publisher_id: u64,
    /// The ISBN of the product, if applicable.
    pub isbn: Option<String>,
    /// The display name of the product.
    pub name: String,
    /// The date the product was purchased (ISO 8601 string), if available.
    #[serde(rename = "datePurchased")]
    pub date_purchased: Option<String>,
    /// The total file size in bytes, if available.
    pub filesize: Option<u64>,
    /// The final price paid for the product.
    #[serde(rename = "finalPrice")]
    pub final_price: f64,
    /// The quantity of this product in the order.
    pub quantity: u32,
    /// The bundle identifier, if the product was purchased as part of a bundle.
    #[serde(rename = "bundleId")]
    pub bundle_id: u64,
    /// Indicates whether the product has been archived (`1`) or not (`0`).
    pub archived: u8,
    /// Additional add-on information associated with this product, if any.
    #[serde(rename = "addOnInfo")]
    pub add_on_info: Option<String>,
    /// The unique identifier for this order-product record.
    #[serde(rename = "orderProductId")]
    pub order_product_id: u64,
    /// The unique identifier of the customer who owns this order.
    #[serde(rename = "customerId")]
    pub customer_id: u64,
    /// The date the product files were last modified (ISO 8601 string), if known.
    #[serde(rename = "fileLastModified")]
    pub file_last_modified: Option<String>,
    /// The date the product files were last downloaded (ISO 8601 string), if known.
    #[serde(rename = "fileLastDownloaded")]
    pub file_last_downloaded: Option<String>,
    /// The list of downloadable files associated with this ordered product.
    pub files: Vec<OrderProductFile>,
    /// Filter categories for this product. Populated when `getFilters=1` is requested.
    pub filters: Option<Vec<OrderProductFilter>>,
    /// The change history for this ordered product, if requested.
    pub history: Option<Vec<OrderProductHistoryEntry>>,
    /// Optional attributes describing purchase options (format, edition, etc.).
    pub attributes: Option<Vec<OrderProductAttribute>>,
}

/// A single item in an ordered products collection response.
///
/// Follows the JSON:API resource object structure with `id`, `type`, and `attributes`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderProductItem {
    /// The JSON:API resource identifier.
    pub id: String,
    /// The JSON:API resource type string (e.g., `"order_product"`).
    #[serde(rename = "type")]
    pub resource_type: String,
    /// The full attribute set for this ordered product.
    pub attributes: OrderProductAttributes,
}

// ── Publisher (included resource) ────────────────────────────────────────────

/// Attributes for a publisher resource included alongside ordered product responses.
///
/// All fields are defaulted because the `included` array may contain mixed resource types
/// (Publisher, Product, Order). Non-publisher items will deserialize with zero/empty values
/// and are filtered out by callers that check `publisher_id > 0`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublisherAttributes {
    /// The display name of the publisher.
    #[serde(default)]
    pub name: String,
    /// The unique identifier of the publisher.
    #[serde(rename = "publisherId", default)]
    pub publisher_id: u64,
    /// The URL slug for the publisher's storefront page.
    #[serde(default)]
    pub slug: String,
}

/// A publisher resource item included in ordered product responses when requested.
///
/// Follows the JSON:API resource object structure.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublisherItem {
    /// The JSON:API resource identifier.
    pub id: String,
    /// The JSON:API resource type string (e.g., `"publisher"`).
    #[serde(rename = "type")]
    pub resource_type: String,
    /// The publisher attributes.
    pub attributes: PublisherAttributes,
}

// ── Response wrappers ─────────────────────────────────────────────────────────

/// A paginated collection of ordered products.
///
/// Returned by `GET /{api_version}/order_products`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderProductListResponse {
    /// Pagination links for navigating the result set.
    pub links: PaginationLinks,
    /// Pagination metadata describing the current page.
    pub meta: PaginationMeta,
    /// The ordered product items on this page.
    pub data: Vec<OrderProductItem>,
    /// Publisher resources included alongside the ordered products, if requested.
    pub included: Option<Vec<PublisherItem>>,
}

/// A single ordered product resource response.
///
/// Returned by `GET /{api_version}/order_products/{id}`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderProductItemResponse {
    /// The ordered product item.
    pub data: OrderProductItem,
}

// ── Product Lists ─────────────────────────────────────────────────────────────

/// Attributes for a product list resource.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProductListAttributes {
    /// The identifier of the customer who owns this list.
    #[serde(rename = "customerId")]
    pub customer_id: u64,
    /// The display name of the product list.
    pub name: String,
    /// The date the list was created (ISO 8601 string).
    #[serde(rename = "dateCreated")]
    pub date_created: String,
    /// The unique identifier for this product list.
    #[serde(rename = "productListId")]
    pub product_list_id: u64,
    /// The URL slug for this product list.
    pub slug: String,
    /// The number of items currently in this product list.
    #[serde(rename = "itemCount")]
    pub item_count: u64,
}

/// A single product list resource item.
///
/// Follows the JSON:API resource object structure.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProductListItem {
    /// The JSON:API resource identifier.
    pub id: String,
    /// The JSON:API resource type string (e.g., `"product_list"`).
    #[serde(rename = "type")]
    pub resource_type: String,
    /// The product list attributes.
    pub attributes: ProductListAttributes,
}

/// A paginated collection of product lists belonging to the authenticated customer.
///
/// Returned by `GET /{api_version}/product_lists`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProductListCollectionResponse {
    /// Pagination links for navigating the result set.
    pub links: PaginationLinks,
    /// Pagination metadata describing the current page.
    pub meta: PaginationMeta,
    /// The product list items on this page.
    pub data: Vec<ProductListItem>,
}

/// A paginated collection of items within a specific product list.
///
/// Returned by `GET /{api_version}/product_list_items`. Individual item schemas are
/// not yet formally defined by the API contract, so items are represented as raw
/// [`serde_json::Value`]s until the schema matures.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProductListItemsResponse {
    /// Pagination links for navigating the result set.
    pub links: PaginationLinks,
    /// Pagination metadata describing the current page.
    pub meta: PaginationMeta,
    /// The raw product list item data on this page.
    pub data: Vec<serde_json::Value>,
}

// ── Query parameter structs ───────────────────────────────────────────────────

/// Query parameters for the `GET /order_products` (library items) endpoint.
///
/// All fields are optional. Set a field to `Some(value)` to include the corresponding
/// query parameter in the request. Use [`Default::default()`] to start with no filters
/// applied.
///
/// # Examples
///
/// ```rust
/// use dtrpg_sdk::LibraryItemsParams;
///
/// let params = LibraryItemsParams {
///     page: Some(2),
///     page_size: Some(50),
///     get_filters: Some(true),
///     ..Default::default()
/// };
/// ```
pub struct LibraryItemsParams {
    /// The page number to retrieve (1-based).
    pub page: Option<u32>,
    /// The number of items to return per page.
    pub page_size: Option<u32>,
    /// When `true`, includes checksum data for each product file (`getChecksum=1`).
    pub get_checksum: Option<bool>,
    /// When `true`, includes filter category data for each product (`getFilters=1`).
    pub get_filters: Option<bool>,
    /// When `true`, restricts results to library (non-archived) products (`library=true`).
    pub library: Option<bool>,
    /// When `true`, includes archived products; when `false`, excludes them (`archived=1/0`).
    pub archived: Option<bool>,
    /// ISO 8601 date string. When set, returns only products updated after this date
    /// (`updatedDate[after]=...`).
    pub updated_date_after: Option<String>,
}

impl Default for LibraryItemsParams {
    fn default() -> Self {
        Self {
            page: None,
            page_size: None,
            get_checksum: None,
            get_filters: None,
            library: None,
            archived: None,
            updated_date_after: None,
        }
    }
}

/// Query parameters for paginated collection endpoints such as `/product_lists`.
///
/// All fields are optional. Use [`Default::default()`] to retrieve the first page with
/// the server's default page size.
///
/// # Examples
///
/// ```rust
/// use dtrpg_sdk::PageParams;
///
/// let params = PageParams { page: Some(3), page_size: Some(25) };
/// ```
pub struct PageParams {
    /// The page number to retrieve (1-based).
    pub page: Option<u32>,
    /// The number of items to return per page.
    pub page_size: Option<u32>,
}

impl Default for PageParams {
    fn default() -> Self {
        Self {
            page: None,
            page_size: None,
        }
    }
}
