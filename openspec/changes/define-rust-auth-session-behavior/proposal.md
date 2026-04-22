## Why

The top-level `dtrpg` repo now has an umbrella auth/session rollout example, and `dtrpg-api` now has a child proposal that defines the API contract side of that work. The Rust SDK repo still needs its own child proposal showing how a language SDK depends on the API contract without redefining it.

## What Changes

- Clarify that the Rust SDK owns Rust-facing authentication and session behavior built on top of the API contract.
- Define how the Rust SDK consumes token lifecycle and auth-failure semantics from `dtrpg-api`.
- Record that this proposal is downstream of the API child proposal `define-auth-session-contract`.
- Strengthen the Rust-specific authentication and configuration capabilities so future implementation work starts from clear ownership boundaries.

## Capabilities

### New Capabilities
- `rust-session-lifecycle`: Defines Rust SDK behavior for holding, invalidating, and reacting to API-defined authentication session state.

### Modified Capabilities
- `rust-authentication-flow`: Clarifies that Rust auth behavior depends on API-owned token and failure semantics.
- `rust-sdk-configuration`: Clarifies that configuration must support the Rust auth/session behavior expected by the SDK.

## Impact

- `dtrpg-sdk/rust/openspec`: New child capability for Rust session lifecycle behavior
- Future Rust SDK source files: Consumers of the API auth/session contract defined upstream
- Upstream dependency: `dtrpg-api/openspec/changes/define-auth-session-contract`
- Umbrella dependency: `dtrpg/openspec/changes/improve-auth-session-lifecycle`
