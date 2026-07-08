//! Library domain: request/response model types and the async [`LibraryClient`].

mod client;
mod models;

pub use client::{ClientError, LibraryClient};
pub use models::{
    FileChecksum, IncludedItem, LibraryItemsParams, OrderProductAttribute, OrderProductAttributes,
    OrderProductDescription, OrderProductFile, OrderProductFilter, OrderProductHistoryEntry,
    OrderProductInfo, OrderProductItem, OrderProductItemResponse, OrderProductListResponse,
    OrderProductOrder, OrderProductPublisher, OrderProductRelationships, PageParams,
    PaginationLinks, PaginationMeta, ProductListAttributes, ProductListCollectionResponse,
    ProductListItem, ProductListItemCreateRequest, ProductListItemCreateResponse,
    ProductListItemsResponse, PublisherAttributes, PublisherItem, RelationshipData,
    RelationshipRef,
};
