## Why

The Rust SDK has authentication and session management, but no support for fetching library
data — ordered products or product lists. Without this capability, consuming applications
cannot use the SDK to browse a user's library at all. They must either implement raw HTTP
themselves (bypassing the SDK entirely) or wait for a future release. Adding library support
makes the SDK useful end-to-end for the most common DriveThruRPG use case: listing and
downloading a customer's purchased content.

This change is a child of the top-level `dtrpg/openspec/changes/add-library-api` umbrella
and a downstream consumer of `dtrpg-api/openspec/changes/define-library-api-contract`.

## What Changes

- **New `rust-library-types` capability**: Rust structs and enums mirroring every
  library-related schema defined by the API contract — ordered products, files, filters,
  history entries, attributes, product lists, and all associated pagination structures.
  Types derive `Deserialize` (and `Serialize`) via `serde` so they can be decoded directly
  from API JSON responses without manual mapping.

- **New `rust-library-client` capability**: An async HTTP client (`LibraryClient`) for all
  library endpoints. It wraps a configured, authenticated `DriveThruRpgSdk` instance and
  exposes one method per API operation. A `DriveThruRpgSdk::library_client()` convenience
  method constructs the client from the current configuration and session.

- **Modified `rust-sdk-configuration`**: `Config` gains an `api_version` field (defaulting
  to `"vBeta"`) so the API version used in URL construction stays configurable without
  breaking existing constructors.

## Capabilities

### New Capabilities
- `rust-library-types`: Rust model types for all API-defined library schemas.
- `rust-library-client`: Async HTTP client for all library API endpoints.

### Modified Capabilities
- `rust-sdk-configuration`: Extended to include a configurable `api_version` value.

## Impact

- `dtrpg-sdk/rust/openspec`: New delta specs for `rust-library-types` and
  `rust-library-client`; updated `rust-sdk-configuration`.
- Rust SDK source files: `library.rs` (model types), `client.rs` (`LibraryClient`),
  `config.rs` (api_version field), `sdk.rs` (`library_client()` method), `lib.rs`
  (module declarations and re-exports).
- Upstream dependencies:
  - `dtrpg-api/openspec/changes/define-library-api-contract` — API schema source of truth.
  - `dtrpg/openspec/changes/add-library-api` — umbrella coordinating this work.
