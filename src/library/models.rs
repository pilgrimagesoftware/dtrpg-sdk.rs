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
    /// Publisher metadata embedded directly on this ordered product's attributes, when the
    /// API includes it inline (in addition to, or instead of, sideloaded `included` publisher
    /// resources).
    #[serde(default)]
    pub publisher: Option<OrderProductPublisher>,
    /// Product catalog metadata (cover images, description) embedded on this ordered product.
    #[serde(default)]
    pub product: Option<OrderProductInfo>,
    /// Order summary metadata embedded on this ordered product.
    #[serde(default)]
    pub order: Option<OrderProductOrder>,
}

/// Publisher metadata embedded directly on an ordered product's attributes.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderProductPublisher {
    /// The display name of the publisher.
    pub name: String,
    /// The unique identifier of the publisher.
    #[serde(rename = "publisherId")]
    pub publisher_id: u64,
    /// The URL slug for the publisher's storefront page.
    pub slug: String,
}

/// Descriptive text for a product, embedded within [`OrderProductInfo`].
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderProductDescription {
    /// The display name of the product.
    pub name: String,
    /// HTML purchase note shown to the customer, if any.
    #[serde(rename = "purchaseNote", default)]
    pub purchase_note: Option<String>,
    /// The URL slug for the product's storefront page.
    pub slug: String,
    /// A short marketing description of the product.
    #[serde(rename = "shortDescription", default)]
    pub short_description: Option<String>,
}

/// Product catalog metadata embedded directly on an ordered product's attributes, including
/// relative paths to cover images.
///
/// Image paths (`image`, `web_image`, `thumbnail`, `thumbnail_100`) are relative to the
/// DriveThruRPG images base URL (`https://api.drivethrurpg.com/images/`).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderProductInfo {
    /// Relative path to the full-size cover image, if available.
    #[serde(default)]
    pub image: Option<String>,
    /// Relative path to the web-optimized (WebP) cover image, if available.
    #[serde(rename = "webImage", default)]
    pub web_image: Option<String>,
    /// Relative path to the 140px cover thumbnail image, if available.
    #[serde(default)]
    pub thumbnail: Option<String>,
    /// Relative path to the 100px cover thumbnail image, if available.
    #[serde(rename = "thumbnail100", default)]
    pub thumbnail_100: Option<String>,
    /// Bundle ID if this product is part of a bundle, otherwise 0.
    #[serde(rename = "bundleId")]
    pub bundle_id: u64,
    /// Date and time when the product was added to the DTRPG catalog, if known.
    #[serde(rename = "dateCreated", default)]
    pub date_created: Option<String>,
    /// Unique identifier for the product in the DTRPG catalog.
    #[serde(rename = "productId")]
    pub product_id: u64,
    /// Descriptive text for the product, if requested.
    #[serde(default)]
    pub description: Option<OrderProductDescription>,
    /// Total file size in megabytes, if known.
    #[serde(default)]
    pub filesize: Option<f64>,
}

/// Order summary metadata embedded on an ordered product's attributes.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderProductOrder {
    /// Date and time when the order was created, if known.
    #[serde(rename = "dateCreated", default)]
    pub date_created: Option<String>,
    /// The unique identifier of the order.
    #[serde(rename = "orderId")]
    pub order_id: u64,
}

/// A single item in an ordered products collection response.
///
/// Follows the JSON:API resource object structure with `id`, `type`, and `attributes`.
///
/// The live API does *not* embed `publisher`/`product`/`order` metadata directly on
/// `attributes` for this endpoint (despite what earlier documentation examples showed) —
/// it references them via `relationships`, resolved against the response's top-level
/// `included` array. See [`OrderProductRelationships`] and [`IncludedItem`].
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderProductItem {
    /// The JSON:API resource identifier.
    pub id: String,
    /// The JSON:API resource type string (e.g., `"order_product"`).
    #[serde(rename = "type")]
    pub resource_type: String,
    /// The full attribute set for this ordered product.
    pub attributes: OrderProductAttributes,
    /// JSON:API relationship references to sideloaded `Publisher`/`Product`/`Order`
    /// resources, resolved by matching `id` against the response's `included` array.
    #[serde(default)]
    pub relationships: Option<OrderProductRelationships>,
}

/// JSON:API relationship references carried on an [`OrderProductItem`].
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderProductRelationships {
    /// Reference to the sideloaded `Publisher` resource, if present.
    #[serde(default)]
    pub publisher: Option<RelationshipRef>,
    /// Reference to the sideloaded `Product` resource, if present.
    #[serde(default)]
    pub product: Option<RelationshipRef>,
    /// Reference to the sideloaded `Order` resource, if present.
    #[serde(default)]
    pub order: Option<RelationshipRef>,
}

/// A single JSON:API relationship reference, wrapping the `data` resource identifier.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RelationshipRef {
    /// The referenced resource's type and id, if the relationship is populated.
    pub data: Option<RelationshipData>,
}

/// The `type`/`id` pair identifying a JSON:API resource referenced by a relationship.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RelationshipData {
    /// The referenced resource's type string (e.g., `"Product"`).
    #[serde(rename = "type")]
    pub resource_type: String,
    /// The referenced resource's id, matching an entry's `id` in the `included` array.
    pub id: String,
}

// ── Sideloaded resources (`included`) ────────────────────────────────────────

/// Attributes for a publisher resource included alongside ordered product responses.
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

/// A single sideloaded resource entity from the `included` array of an ordered-products
/// list response.
///
/// The `included` array mixes multiple JSON:API resource types (`Publisher`, `Product`,
/// `Order`) in a single flat list; `resource_type` disambiguates which, and `attributes`
/// is kept as an untyped [`serde_json::Value`] since its shape depends on `resource_type`.
/// Decode it via [`IncludedItem::as_publisher`] or [`IncludedItem::as_product`].
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IncludedItem {
    /// The JSON:API resource identifier. Matches a [`RelationshipData::id`] referencing it.
    pub id: String,
    /// The JSON:API resource type string (e.g., `"Publisher"`, `"Product"`, `"Order"`).
    #[serde(rename = "type")]
    pub resource_type: String,
    /// The resource's untyped attribute payload; shape depends on `resource_type`.
    pub attributes: serde_json::Value,
}

impl IncludedItem {
    /// Decodes `attributes` as [`PublisherAttributes`] if `resource_type == "Publisher"`.
    ///
    /// Returns `None` for any other resource type or if decoding fails.
    #[must_use]
    pub fn as_publisher(&self) -> Option<PublisherAttributes> {
        if self.resource_type != "Publisher" {
            return None;
        }
        serde_json::from_value(self.attributes.clone()).ok()
    }

    /// Decodes `attributes` as [`OrderProductInfo`] if `resource_type == "Product"`.
    ///
    /// Returns `None` for any other resource type or if decoding fails.
    #[must_use]
    pub fn as_product(&self) -> Option<OrderProductInfo> {
        if self.resource_type != "Product" {
            return None;
        }
        serde_json::from_value(self.attributes.clone()).ok()
    }
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
    /// Publisher/Product/Order resources sideloaded alongside the ordered products.
    pub included: Option<Vec<IncludedItem>>,
}

/// A single ordered product resource response.
///
/// Returned by `GET /{api_version}/order_products/{id}`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderProductItemResponse {
    /// The ordered product item.
    pub data: OrderProductItem,
    /// Publisher/Product/Order resources sideloaded alongside the ordered product,
    /// resolved by matching `relationships.*.data.id` against each entry's `id`
    /// (mirrors [`OrderProductListResponse::included`]).
    #[serde(default)]
    pub included: Option<Vec<IncludedItem>>,
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

/// Request body for adding a product to a product list.
///
/// Sent by `POST /{api_version}/product_list_items`.
#[derive(Clone, Debug, Serialize)]
pub struct ProductListItemCreateRequest {
    /// Unique identifier of the product to add.
    #[serde(rename = "productId")]
    pub product_id: u64,
    /// Unique identifier of the product list to add the product to.
    #[serde(rename = "productListId")]
    pub product_list_id: u64,
}

/// The created product list item.
///
/// Returned by `POST /{api_version}/product_list_items`. The API wraps this resource
/// in a JSON:API-style envelope on the wire (`{"data": {"id": ..., "type": ...,
/// "attributes": {"productId": ..., "productListId": ..., "productListItemId": ...}}}`);
/// [`Deserialize`] unwraps that envelope so callers work with a flat struct.
#[derive(Clone, Debug, Serialize)]
pub struct ProductListItemCreateResponse {
    /// Unique identifier of the product added to the list.
    pub product_id: u64,
    /// Unique identifier of the product list the product was added to.
    pub product_list_id: u64,
    /// Unique identifier assigned to this product list item. Required to remove
    /// the item later via `DELETE /{api_version}/product_list_items/{id}`.
    pub product_list_item_id: u64,
}

impl<'de> Deserialize<'de> for ProductListItemCreateResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Attributes {
            #[serde(rename = "productId")]
            product_id: u64,
            #[serde(rename = "productListId")]
            product_list_id: u64,
            #[serde(rename = "productListItemId")]
            product_list_item_id: u64,
        }
        #[derive(Deserialize)]
        struct Resource {
            attributes: Attributes,
        }
        #[derive(Deserialize)]
        struct Envelope {
            data: Resource,
        }

        let envelope = Envelope::deserialize(deserializer)?;
        Ok(Self {
            product_id: envelope.data.attributes.product_id,
            product_list_id: envelope.data.attributes.product_list_id,
            product_list_item_id: envelope.data.attributes.product_list_item_id,
        })
    }
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
#[derive(Default)]
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
#[derive(Default)]
pub struct PageParams {
    /// The page number to retrieve (1-based).
    pub page: Option<u32>,
    /// The number of items to return per page.
    pub page_size: Option<u32>,
}
