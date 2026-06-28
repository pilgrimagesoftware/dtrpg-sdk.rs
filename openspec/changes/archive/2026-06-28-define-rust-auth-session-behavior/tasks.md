## 1. Rust SDK Specs

- [x] 1.1 Add the `rust-session-lifecycle` delta spec in `dtrpg-sdk/rust`
- [x] 1.2 Update `rust-authentication-flow` to depend on the API auth/session contract
- [x] 1.3 Update `rust-sdk-configuration` to reflect Rust auth/session setup needs

## 2. Upstream Alignment

- [x] 2.1 Reference `define-auth-session-contract` as the API dependency for Rust auth/session behavior
- [x] 2.2 Confirm the Rust SDK proposal does not redefine API token or error semantics

## 3. Future Implementation Follow-Through

- [x] 3.1 Introduce Rust SDK types and modules that satisfy the approved auth/session behavior
- [x] 3.2 Validate that future Rust auth/session implementation preserves the API-defined lifecycle meaning
