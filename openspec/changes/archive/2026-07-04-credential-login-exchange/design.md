## Context

`dtrpg-api/LOGIN.md` documents the two website endpoints DriveThruRPG's own
login page uses:

1. `POST https://www.drivethrurpg.com/validate_login_credentials.php` —
   validates credentials, returns a JSON-encoded array (e.g.
   `["password",true,"Locked",true]`), not an object.
2. `POST https://www.drivethrurpg.com/create_account_app.php` — same request
   shape, returns `{"status": "success", "message": {"key": "<application-key>"}}`.

Both are `www.drivethrurpg.com` website form endpoints, not part of the
`dtrpg-api` OpenAPI contract for `api.drivethrurpg.com`.

## Goals / Non-Goals

**Goals:**
- Wrap both calls behind one SDK function so callers issue a single logical
  "log in with credentials" call.
- Keep the existing `auth_client::authenticate` (`auth_key` → JWT exchange)
  untouched; this module only produces the application key.

**Non-Goals:**
- Do not add these endpoints to `openapi.yaml`.
- Do not attempt SSO, OAuth, or browser-based login.

## Decisions

- **New module `credential_login.rs`, not a change to `auth_client.rs`**:
  targets a different host (`www.drivethrurpg.com` vs `config.base_url()`)
  and a non-uniform response shape. Mixing it into `auth_client.rs` would blur
  that module's `api.drivethrurpg.com`-only scope.
- **Sequential two-request implementation, first response is a status gate**:
  call `validate_login_credentials.php` first; only call
  `create_account_app.php` if the first response indicates valid credentials.
  Both requests use the same `multipart/form-data` body
  (`email_address`, `password`) per `LOGIN.md`.
- **Typed positional response**: `validate_login_credentials.php`'s bare JSON
  array is deserialized into a small struct with named accessors
  (`ValidateLoginResponse { field: String, ok: bool, message: String, locked: bool }`
  or similar), documented in a code comment next to the deserializer with the
  exact field order from `LOGIN.md`, and covered by a unit test fixture.
- **Error surface**: returns `ClientError` (the existing SDK error enum), so
  callers get `ClientError::Http` for transport failures and a distinguishable
  variant/message for invalid credentials vs. a key-request failure after
  valid credentials, so `dtrpg-app` can show the right error to the user.

## Risks / Trade-offs

- Website endpoints are not versioned or officially supported like
  `api.drivethrurpg.com`, so DriveThruRPG could change them without notice →
  isolated behind this module; `LOGIN.md` remains the source of truth to diff
  against.
- The positional array response is fragile → documented field order plus a
  fixture-based unit test using the exact `LOGIN.md` example.

## Migration Plan

1. Add typed responses for both endpoints with unit test fixtures.
2. Implement `login_with_credentials`.
3. Add unit tests for valid credentials, invalid credentials (stop before
   second call), and key-request failure after valid credentials.
4. Document scope relative to `auth_client.rs`.
