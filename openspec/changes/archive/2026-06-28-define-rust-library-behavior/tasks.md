## 1. Rust SDK Specs

- [x] 1.1 Add the `rust-library-types` delta spec in `dtrpg-sdk/rust`
- [x] 1.2 Add the `rust-library-client` delta spec in `dtrpg-sdk/rust`
- [x] 1.3 Update `rust-sdk-configuration` delta spec to document the `api_version` field

## 2. Upstream Alignment

- [x] 2.1 Reference `define-library-api-contract` as the API dependency for Rust library behavior
- [x] 2.2 Confirm Rust types derive their structure from API-defined schemas rather than local guesses

## 3. Implementation

- [x] 3.1 Add `reqwest` (0.12, `json` feature), `serde` (1, `derive` feature), and `serde_json` (1) to `Cargo.toml` dependencies
- [x] 3.2 Add `tokio` (1, `rt-multi-thread` + `macros` features) to `Cargo.toml` dev-dependencies
- [x] 3.3 Implement `library.rs` with Rust model types for all API-defined library schemas
- [x] 3.4 Implement `client.rs` with `LibraryClient` struct, `ClientError` enum, and all endpoint methods
- [x] 3.5 Update `config.rs` to add `api_version: String` field (default `"vBeta"`) and `api_version()` accessor
- [x] 3.6 Update `sdk.rs` to add `library_client()` convenience method
- [x] 3.7 Update `lib.rs` to declare `client` and `library` modules and re-export all public types
