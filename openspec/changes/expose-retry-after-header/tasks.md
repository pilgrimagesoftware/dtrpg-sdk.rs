## 1. ClientError

- [x] 1.1 In `src/library/client.rs`, add `retry_after: Option<std::time::Duration>`
      to `ClientError::ApiError`
- [x] 1.2 Update `ClientError`'s `Display` impl's `ApiError` arm (and any other
      exhaustive match on the variant's fields within this crate) to account
      for the new field

## 2. Header Parsing

- [x] 2.1 In `decode_response`, before calling `response.bytes()`, read
      `response.headers().get(reqwest::header::RETRY_AFTER)` and parse it as
      `delay-seconds` (`to_str().ok().and_then(|s|
      s.trim().parse::<u64>().ok()).map(Duration::from_secs)`), defaulting to
      `None` on any absence or parse failure
- [x] 2.2 Pass the parsed value into the `ClientError::ApiError` constructed
      in the `!status.is_success()` branch

## 3. Tests

- [x] 3.1 Unit/integration test (using the existing `wiremock` pattern):
      a mocked 429 response with `Retry-After: 30` yields
      `ApiError { retry_after: Some(Duration::from_secs(30)), .. }`
- [x] 3.2 Test: a mocked non-success response with no `Retry-After` header
      yields `retry_after: None`
- [x] 3.3 Test: a mocked non-success response with an unparseable
      `Retry-After` value (e.g. an HTTP-date string) yields `retry_after:
      None` without erroring
- [x] 3.4 Confirm an existing `ApiError`-asserting test (e.g.
      `add_product_list_item_returns_api_error_on_failure_status`) still
      passes unmodified aside from any exhaustive-match updates from task 1.2

## 4. Build and Quality

- [x] 4.1 `cargo check --all-targets`
- [x] 4.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [x] 4.3 `cargo test --all-features --workspace`
- [x] 4.4 `cargo fmt --all -- --check`
- [ ] 4.5 `cargo doc --no-deps -- -D warnings` — pre-existing failure on
      `develop`, unrelated to this change: `LOG_PAYLOAD_LIMIT` is a private
      const referenced from public doc comments on `DecodeFailed`/`ApiError`
      predating this change. Out of scope; needs its own fix.
