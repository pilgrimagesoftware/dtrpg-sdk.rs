## ADDED Requirements

### Requirement: The Rust SDK MUST provide an async HTTP client for all library endpoints
The Rust SDK MUST expose an async client that callers can use to fetch ordered products,
product details, download preparation, product lists, and product list items from the
DriveThruRPG API.

#### Scenario: Fetching the user's library
- **WHEN** a caller invokes the Rust SDK to list a user's ordered products
- **THEN** the SDK sends the authenticated request and returns deserialized Rust types

### Requirement: The Rust library client MUST be created from a configured, authenticated SDK instance
The `LibraryClient` MUST require both SDK configuration and an active authentication
session before it can make API calls.

#### Scenario: Creating a library client from the SDK
- **WHEN** a caller creates a `LibraryClient` from a `DriveThruRpgSdk` instance
- **THEN** the SDK surfaces an error if the instance is unconfigured or unauthenticated

### Requirement: The Rust library client MUST authenticate requests using both the application key and the bearer token
Every library API request MUST include the `applicationKey` query parameter and the
`Authorization: Bearer <token>` header as required by the API contract.

#### Scenario: Sending an authenticated library request
- **WHEN** the Rust library client sends a request to any library endpoint
- **THEN** the request includes both the application key and the bearer token from the
  active session
