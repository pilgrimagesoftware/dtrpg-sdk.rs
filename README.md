# DriveThruRPG SDK (Rust)

A Rust SDK for the [DriveThruRPG API](https://api.drivethrurpg.com).

Provides configuration, authentication/session lifecycle, and an async library client for
listing orders, product lists, and preparing downloads.

## Installation

```bash
cargo add dtrpg-sdk
```

## Quick Start

```rust
use dtrpg_sdk::{Config, DriveThruRpgSdk, AuthTokenResponse};

let mut sdk = DriveThruRpgSdk::with_config(Config::new("my-app-key"));

// After receiving an auth response from the API:
let response = AuthTokenResponse::new("jwt-token", "refresh-token", 1_800_000_000);
let session = sdk.apply_auth_response(response).unwrap();
assert_eq!(session.token(), "jwt-token");

// Create an authenticated library client:
let client = sdk.library_client().unwrap();
// client.list_order_products(Default::default()).await ...
```

See the crate's [rustdoc](https://docs.rs/dtrpg-sdk) for the full API reference, including
`Config`, `AuthSession`/`AuthState`, `LibraryClient`, and the library model types
(`OrderProductItem`, `ProductListItem`, etc.).

## OpenAPI Source

The crate uses the `dtrpg-api` repository as an `API` submodule. `build.rs` reads
`API/openapi.yaml` during compilation and generates Rust OpenAPI metadata into Cargo's
`OUT_DIR`, exposed through `dtrpg_sdk::openapi`.

## Development

```bash
cargo check --all-targets
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
cargo test --all-features --workspace
cargo doc --no-deps -- -D warnings
```

## Release Process

See [RELEASE.md](https://github.com/pilgrimagesoftware/dtrpg-sdk.rs/blob/master/RELEASE.md).

## License

MIT — see [LICENSE.md](LICENSE.md).
