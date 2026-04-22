## Context

The Rust SDK repository is still minimal, but that makes its ownership boundary especially important to define early. The API repository owns token issuance, refresh semantics, expiry meaning, and auth-related failures. The Rust SDK should consume those semantics and present them in Rust-idiomatic configuration, types, and control flow rather than inventing parallel meanings.

## Goals / Non-Goals

**Goals:**
- Show how a Rust SDK child proposal depends on the API contract child proposal.
- Define what auth/session behavior belongs in the Rust SDK instead of the API repository or the top-level coordinating repo.
- Establish a reusable pattern for future Rust implementation work.

**Non-Goals:**
- Change the API contract itself
- Define desktop or application UX for session expiry
- Commit to specific Rust types or module layouts before implementation exists

## Decisions

Create a dedicated `rust-session-lifecycle` capability.
Rationale: Rust SDK behavior around holding and reacting to session state is a distinct concern from generic authentication flow and should be easy to discover later.

Modify `rust-authentication-flow` to explicitly depend on API-owned semantics.
Rationale: the SDK should adapt contract meaning into Rust behavior, not redefine token lifecycle or error meaning locally.

Modify `rust-sdk-configuration` to reflect auth/session needs.
Rationale: configuration is the entry point for SDK behavior, so the spec should make clear that auth/session behavior starts with explicit configuration.

## Risks / Trade-offs

- The proposal is ahead of the current codebase -> Mitigation: keep it focused on owned behavior and dependency direction rather than implementation details.
- Future Rust implementation choices may shift naming or structure -> Mitigation: the specs define behavior and ownership, not concrete file layout.
- Downstream app repos could still bypass the SDK and interpret API auth behavior directly -> Mitigation: keep the SDK proposal clear about which Rust-facing semantics it intends to own.
