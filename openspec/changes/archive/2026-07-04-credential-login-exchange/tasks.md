## 1. Typed responses

- [x] 1.1 Add a typed response for `validate_login_credentials.php`'s
      positional JSON array, with a unit test fixture from the exact example
      in `LOGIN.md`
- [x] 1.2 Add a typed response for `create_account_app.php`'s
      `{status, message.key}` body

## 2. Credential exchange function

- [x] 2.1 Implement `login_with_credentials(email, password, config) ->
      Result<String, ClientError>` calling both endpoints in sequence against
      `www.drivethrurpg.com`
- [x] 2.2 Add unit tests covering: valid credentials, invalid credentials
      (stop before second call), and key-request failure after valid
      credentials
- [x] 2.3 Document the new module's scope relative to `auth_client.rs` in doc
      comments

## 3. Verification

- [x] 3.1 Run `cargo test --workspace`
- [x] 3.2 Run `cargo clippy --all-targets --all-features -- -D warnings`
