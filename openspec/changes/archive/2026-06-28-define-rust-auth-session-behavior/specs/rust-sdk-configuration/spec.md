## MODIFIED Requirements

### Requirement: Rust SDK configuration must be explicit before use
The Rust SDK MUST require the configuration values needed to initialize its client behavior before authenticated operations are attempted, including the values needed to drive Rust-owned auth/session behavior.

#### Scenario: Attempting to use the SDK without configuration
- **WHEN** a caller invokes authenticated Rust SDK behavior before providing the required configuration
- **THEN** the SDK surfaces the documented unconfigured or uninitialized behavior

#### Scenario: Providing configuration for auth/session behavior
- **WHEN** a caller configures the Rust SDK for authenticated API use
- **THEN** the Rust SDK has the configuration needed to apply its documented authentication and session lifecycle behavior
