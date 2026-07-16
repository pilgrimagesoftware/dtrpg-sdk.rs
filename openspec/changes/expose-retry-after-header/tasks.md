## 1. ClientError

- [ ] 1.1 In `src/library/client.rs`, add `retry_after: Option<std::time::Duration>`
      to `ClientError::ApiError`
- [ ] 1.2 Update `ClientError`'s `Display` impl's `ApiError` arm (and any other
      exhaustive match on the variant's fields within this crate) to account
      for the new field

## 2. Header Parsing

- [ ] 2.1 In `decode_response`, before calling `response.bytes()`, read
      `response.headers().get(reqwest::header::RETRY_AFTER)` and parse it as
      `delay-seconds` (`to_str().ok().and_then(|s|
      s.trim().parse::<u64>().ok()).map(Duration::from_secs)`), defaulting to
      `None` on any absence or parse failure
- [ ] 2.2 Pass the parsed value into the `ClientError::ApiError` constructed
      in the `!status.is_success()` branch

## 3. Tests

- [ ] 3.1 Unit/integration test (using the existing `wiremock` pattern):
      a mocked 429 response with `Retry-After: 30` yields
      `ApiError { retry_after: Some(Duration::from_secs(30)), .. }`
- [ ] 3.2 Test: a mocked non-success response with no `Retry-After` header
      yields `retry_after: None`
- [ ] 3.3 Test: a mocked non-success response with an unparseable
      `Retry-After` value (e.g. an HTTP-date string) yields `retry_after:
      None` without erroring
- [ ] 3.4 Confirm an existing `ApiError`-asserting test (e.g.
      `add_product_list_item_returns_api_error_on_failure_status`) still
      passes unmodified aside from any exhaustive-match updates from task 1.2

## 4. Build and Quality

- [ ] 4.1 `cargo check --all-targets`
- [ ] 4.2 `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] 4.3 `cargo test --all-features --workspace`
- [ ] 4.4 `cargo fmt --all -- --check`
- [ ] 4.5 `cargo doc --no-deps -- -D warnings`
