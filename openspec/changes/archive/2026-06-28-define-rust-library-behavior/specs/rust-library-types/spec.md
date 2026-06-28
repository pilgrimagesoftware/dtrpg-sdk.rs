## ADDED Requirements

### Requirement: The Rust SDK MUST define typed models for all API-defined library schemas
The Rust SDK MUST provide Rust structs or enums for each library resource schema defined
by the API contract, including ordered products, files, filters, history entries,
attributes, product lists, and pagination structures.

#### Scenario: Deserializing an ordered product response
- **WHEN** the Rust SDK receives an ordered product response from the API
- **THEN** it deserializes it into the documented Rust model types derived from the
  API-defined schemas

### Requirement: Rust library types MUST derive their structure from API contract schemas
Rust library model types MUST follow the field names, optionality, and nesting defined by
the API repository rather than introducing SDK-local interpretations.

#### Scenario: Adding a new field to a library resource
- **WHEN** the API contract adds a new field to an ordered product or product list schema
- **THEN** the Rust SDK type is updated to reflect the API-defined change rather than a
  local guess
