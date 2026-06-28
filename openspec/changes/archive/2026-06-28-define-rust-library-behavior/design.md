## Context

The Rust SDK currently has zero HTTP dependencies and manages only session state. The
existing modules (`config`, `auth`, `error`, `sdk`) provide a solid foundation, but the
SDK cannot actually call the DriveThruRPG API on behalf of a caller.

Adding library access introduces two concrete requirements that did not previously exist:

1. **HTTP**: The SDK must send real HTTP requests to the DriveThruRPG API.
2. **JSON deserialization**: The SDK must decode API responses into typed Rust structs.

The API authenticates library requests using two mechanisms simultaneously: an
`applicationKey` query parameter (identifies the publisher/application) and an
`Authorization: Bearer <token>` header (authenticates the customer session). Both must
be present on every library request.

This change builds directly on the API contract defined in
`dtrpg-api/openspec/changes/define-library-api-contract` and is coordinated by the
umbrella change `dtrpg/openspec/changes/add-library-api`.

## Goals / Non-Goals

**Goals:**
- Rust-idiomatic typed structs for every library schema defined by the API contract.
- An async HTTP client (`LibraryClient`) covering all library API endpoints.
- A `DriveThruRpgSdk::library_client()` convenience method that validates configuration
  and session state before constructing the client.
- A configurable `api_version` on `Config` (default `"vBeta"`) to keep URL construction
  flexible without hardcoding the version string.

**Non-Goals:**
- Synchronous HTTP — the Rust ecosystem has settled on async as the primary model for
  network I/O; a sync wrapper is a separate concern.
- Response caching — cache policy belongs to the consuming application, not the SDK.
- File download I/O — `prepare_download` returns the API's response payload; writing
  bytes to disk is outside the SDK's scope.

## Decisions

### Use `reqwest` 0.12 with the `json` feature for async HTTP
**Rationale:** `reqwest` is the de-facto standard async HTTP client in the Rust ecosystem.
Version 0.12 is current and targets `tokio`. The `json` feature pulls in `serde_json`
integration so `.json::<T>()` deserialization is available on responses without extra
glue code.

### Use `serde` with the `derive` feature for deserialization
**Rationale:** All model types need to implement `Deserialize`. The `derive` feature lets
the compiler generate the implementation from field-level annotations, keeping the types
declarative and easy to maintain as the API schema evolves.

### Define `LibraryClient` as a separate struct created from a configured, authenticated SDK
**Rationale:** Bundling HTTP into `DriveThruRpgSdk` directly would couple the session
lifecycle struct to a `reqwest::Client`. Keeping `LibraryClient` separate follows the
single-responsibility principle and makes the SDK easier to test — callers can construct
a `LibraryClient` with arbitrary configuration and tokens without going through the full
SDK initialization path.

### Add `api_version` to `Config` (default `"vBeta"`)
**Rationale:** Hardcoding `"vBeta"` directly in URL construction would make it impossible
to target future or staging API versions without a code change. Putting it on `Config`
keeps the version string observable and overridable. The existing `Config::new` and
`Config::with_base_url` constructors are not changed; `api_version` defaults to `"vBeta"`
in both.

### Treat the `prepare-download` response as `serde_json::Value`
**Rationale:** The API contract for the `prepare` endpoint has not yet been formally
defined in `dtrpg-api`. Using `serde_json::Value` is the most honest representation of
that uncertainty — it deserializes correctly regardless of the actual payload shape, and
can be tightened to a concrete type once the contract matures.

## Risks / Trade-offs

- **`reqwest` dependency tree**: `reqwest` transitively brings in `hyper`, `tokio-tls`,
  and related crates, which increases compile time and binary size. Mitigation: this is
  unavoidable for a network-capable SDK, and `reqwest` is already the ecosystem standard.
- **`prepare` endpoint is weakly typed**: Returning `serde_json::Value` for the prepare
  response sacrifices type safety for that one endpoint. Mitigation: this is explicitly
  acknowledged in the spec and in source-level documentation; a future change can
  strengthen the type once the API contract is stable.
- **No retry or error recovery**: `LibraryClient` propagates `reqwest::Error` directly.
  Mitigation: retry logic is application-level policy and should not be baked into the
  SDK's first library client iteration.
