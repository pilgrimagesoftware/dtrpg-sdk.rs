## ADDED Requirements

### Requirement: SDK exchanges credentials for an application key
The Rust SDK SHALL expose `login_with_credentials(email, password, config)`
that exchanges an email/password pair for a DriveThruRPG application key by
calling `validate_login_credentials.php` then `create_account_app.php` on
`www.drivethrurpg.com`, without requiring the caller to construct either
request directly.

#### Scenario: Valid credentials return an application key
- **WHEN** `login_with_credentials` is called with an email and password that
  DriveThruRPG accepts
- **THEN** it calls `validate_login_credentials.php`, confirms the response
  indicates valid credentials, calls `create_account_app.php`, and returns the
  application key from `message.key`

#### Scenario: Invalid credentials stop before the key request
- **WHEN** `validate_login_credentials.php` indicates invalid credentials
- **THEN** `login_with_credentials` returns an error without calling
  `create_account_app.php`

#### Scenario: Key request fails after valid credentials
- **WHEN** `validate_login_credentials.php` indicates valid credentials but
  `create_account_app.php` returns an error or unexpected body
- **THEN** `login_with_credentials` returns an error distinguishable from the
  invalid-credentials case

### Requirement: Positional validation response is typed
The SDK SHALL deserialize `validate_login_credentials.php`'s bare JSON array
response into a named struct with documented field order, rather than
requiring callers to index into a raw array.

#### Scenario: Parsing the documented example response
- **WHEN** the SDK receives the exact example body from `LOGIN.md`
  (`["password",true,"Locked",true]`)
- **THEN** it parses successfully into the typed struct with each field
  accessible by name

### Requirement: Credential exchange is scoped separately from the API auth client
The SDK SHALL implement the website credential exchange in a module distinct
from `auth_client.rs`, and SHALL document in that module's doc comments that
it targets `www.drivethrurpg.com` rather than `api.drivethrurpg.com` and does
not replace or modify `auth_client::authenticate`.

#### Scenario: Existing auth_key exchange is unaffected
- **WHEN** a caller uses `auth_client::authenticate` with an application key
- **THEN** its behavior and signature are unchanged by the addition of
  `login_with_credentials`
