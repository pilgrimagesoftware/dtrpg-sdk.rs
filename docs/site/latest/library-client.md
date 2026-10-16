---
title: Library client
version: latest
slug: library-client
---

# Library client

`sdk.library_client()` returns a `LibraryClient` once a session is authenticated. It wraps
`reqwest` and exposes one async method per API operation.

## Ordered products

```rust,no_run
use dtrpg_sdk::{Config, DriveThruRpgSdk, AuthTokenResponse, LibraryItemsParams};

# async fn run() -> Result<(), dtrpg_sdk::ClientError> {
let mut sdk = DriveThruRpgSdk::with_config(Config::new("my-app-key"));
sdk.apply_auth_response(AuthTokenResponse::new("jwt-token", "refresh-token", 9_999_999_999)).unwrap();
let client = sdk.library_client().unwrap();

// Page through purchased products.
let params = LibraryItemsParams { page: Some(1), page_size: Some(25), ..Default::default() };
let products = client.list_order_products(params).await?;

// Fetch a single ordered product by id.
let product = client.get_order_product(515_276).await?;

// Prepare a download link for a specific file index on an ordered product.
let download = client.prepare_download(515_276, 0).await?;
# Ok(())
# }
```

## Product lists

```rust,no_run
use dtrpg_sdk::{Config, DriveThruRpgSdk, AuthTokenResponse, PageParams};

# async fn run() -> Result<(), dtrpg_sdk::ClientError> {
let mut sdk = DriveThruRpgSdk::with_config(Config::new("my-app-key"));
sdk.apply_auth_response(AuthTokenResponse::new("jwt-token", "refresh-token", 9_999_999_999)).unwrap();
let client = sdk.library_client().unwrap();

let lists = client.list_product_lists(PageParams::default()).await?;
let list = client.create_product_list("Wishlist").await?;
let items = client.list_product_list_items(86_151, PageParams::default()).await?;

let item = client.add_product_list_item(86_151, 515_276).await?;
client.delete_product_list_item(item.product_list_item_id).await?;
# Ok(())
# }
```

See [docs.rs/dtrpg-sdk](https://docs.rs/dtrpg-sdk) for the full method reference and model
types (`OrderProductItem`, `ProductListItem`, pagination types, and errors).
