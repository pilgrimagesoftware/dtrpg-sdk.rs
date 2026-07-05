## Why

The DriveThruRPG desktop apps ask users to paste a raw application key with no
supported way to discover one. `dtrpg-api/LOGIN.md` documents the two website
endpoints DriveThruRPG's own login page uses to turn an email/password pair
into an application key. The Rust SDK needs a function that wraps those two
calls so app code issues one logical "log in" call instead of talking to
`www.drivethrurpg.com` directly.

This is a child change of the umbrella proposal
`dtrpg/openspec/changes/replace-api-key-entry-with-credential-login`.

## What Changes

- Add a new module, separate from `auth_client.rs`, that calls
  `validate_login_credentials.php` then `create_account_app.php` in sequence
  and returns the application key.
- Add typed responses for both endpoints, including the positional JSON array
  returned by `validate_login_credentials.php`.
- Add `login_with_credentials(email, password, config) -> Result<String, ClientError>`.

## Capabilities

### New Capabilities
- `credential-login-exchange`: SDK-level operation that exchanges an
  email/password pair for a DriveThruRPG application key via the website
  login endpoints.

## Impact

- `dtrpg-sdk/rust/src`: new module for the website credential exchange,
  separate from the existing `api.drivethrurpg.com` auth client
  (`auth_client.rs`).
- No changes to `dtrpg-api`'s `openapi.yaml` — these endpoints are outside the
  `api.drivethrurpg.com` contract.
- Upstream/umbrella dependency: `dtrpg/openspec/changes/replace-api-key-entry-with-credential-login`.
