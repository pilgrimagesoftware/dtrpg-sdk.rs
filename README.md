# dtrpg-sdk.rs

An SDK for DriveThru RPG API in Rust

## OpenAPI source

The crate keeps a local `API/openapi.yaml` copy of the DriveThruRPG API contract.
`build.rs` reads that file during compilation and generates Rust OpenAPI metadata
into Cargo's `OUT_DIR`, exposed through `dtrpg_sdk::openapi`.
