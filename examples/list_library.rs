//! Authenticates with an app key and JWT token and prints the first page of ordered
//! products in the user's library.
//!
//! ```bash
//! DTRPG_APP_KEY=my-app-key DTRPG_JWT_TOKEN=jwt-token DTRPG_REFRESH_TOKEN=refresh-token \
//!   cargo run --example list_library
//! ```

use dtrpg_sdk::{AuthTokenResponse, Config, DriveThruRpgSdk, LibraryItemsParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_key = std::env::var("DTRPG_APP_KEY")?;
    let token = std::env::var("DTRPG_JWT_TOKEN")?;
    let refresh_token = std::env::var("DTRPG_REFRESH_TOKEN")?;

    let mut sdk = DriveThruRpgSdk::with_config(Config::new(app_key));
    sdk.apply_auth_response(AuthTokenResponse::new(token, refresh_token, u64::MAX))?;

    let client = sdk.library_client()?;
    let params = LibraryItemsParams {
        page: Some(1),
        page_size: Some(25),
        ..Default::default()
    };
    let products = client.list_order_products(params).await?;

    for item in products.data {
        println!(
            "{} (order {})",
            item.attributes.name, item.attributes.order_id
        );
    }

    Ok(())
}
