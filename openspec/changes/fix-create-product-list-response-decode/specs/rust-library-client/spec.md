## ADDED Requirements

### Requirement: The Rust library client MUST decode the `data`-enveloped JSON:API response returned when creating a product list
`POST /{api_version}/product_lists` returns the created resource wrapped in a JSON:API
`data` envelope (`{"data": {"id": ..., "type": ..., "attributes": {...}}}`), not the
resource object at the payload's top level. `create_product_list` MUST decode the envelope
and return the unwrapped `ProductListItem` to callers.

#### Scenario: Creating a product list decodes the enveloped response
- **WHEN** a caller invokes `create_product_list` and the API responds with
  `{"data": {"id": "/api/vBeta/product_lists/86267", "type": "ProductList", "attributes": {"customerId": 399144, "name": "Testing", "dateCreated": "2026-07-09T00:42:39-05:00", "productListId": 86267, "slug": "testing", "itemCount": 0}}}`
- **THEN** the SDK returns `Ok(ProductListItem)` populated from the `data` object, rather
  than a `ClientError::DecodeFailed` for a missing `id` field
