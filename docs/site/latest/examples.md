---
title: Examples
version: latest
slug: examples
---

# Examples

Runnable examples live in the repository's [`examples/`](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/tree/develop/examples)
directory and run via `cargo run --example <name>`.

## `list_library`

Authenticates with an app key and JWT token (read from environment variables) and prints
the first page of ordered products.

```bash
DTRPG_APP_KEY=my-app-key DTRPG_JWT_TOKEN=jwt-token DTRPG_REFRESH_TOKEN=refresh-token \
  cargo run --example list_library
```

```rust,ignore
use dtrpg_sdk::{AuthTokenResponse, Config, DriveThruRpgSdk, LibraryItemsParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_key = std::env::var("DTRPG_APP_KEY")?;
    let token = std::env::var("DTRPG_JWT_TOKEN")?;
    let refresh_token = std::env::var("DTRPG_REFRESH_TOKEN")?;

    let mut sdk = DriveThruRpgSdk::with_config(Config::new(app_key));
    sdk.apply_auth_response(AuthTokenResponse::new(token, refresh_token, u64::MAX))?;

    let client = sdk.library_client()?;
    let params = LibraryItemsParams { page: Some(1), page_size: Some(25), ..Default::default() };
    let products = client.list_order_products(params).await?;

    for item in products.data {
        println!("{} (order {})", item.attributes.name, item.attributes.order_id);
    }

    Ok(())
}
```

For more usage patterns, see [Authentication](../authentication/) and
[Library client](../library-client/).
