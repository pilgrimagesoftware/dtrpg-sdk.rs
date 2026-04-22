## Purpose
Define how the Rust SDK accepts and applies caller configuration so client initialization stays explicit, Rust-idiomatic, and aligned with the underlying API contract.

## Requirements

### Requirement: Rust SDK configuration must be explicit before use
The Rust SDK MUST require the configuration values needed to initialize its client behavior before authenticated operations are attempted.

#### Scenario: Attempting to use the SDK without configuration
- **WHEN** a caller invokes authenticated Rust SDK behavior before providing the required configuration
- **THEN** the SDK surfaces the documented unconfigured or uninitialized behavior

### Requirement: Rust SDK configuration must remain Rust-idiomatic
The Rust SDK MUST expose configuration in a form that fits Rust conventions while preserving the underlying API contract requirements.

#### Scenario: Configuring a custom API endpoint
- **WHEN** a caller provides a supported custom base URL or application key through the Rust SDK configuration surface
- **THEN** the SDK applies that configuration using the documented Rust-oriented configuration model
