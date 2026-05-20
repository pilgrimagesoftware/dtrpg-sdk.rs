# dtrpg-sdk.rs

An SDK for DriveThru RPG API in Rust

## OpenAPI source

The crate uses the `dtrpg-api` repository as an `API` submodule. `build.rs` reads `API/openapi.yaml` during compilation
and generates Rust OpenAPI metadata into Cargo's `OUT_DIR`, exposed through `dtrpg_sdk::openapi`.
