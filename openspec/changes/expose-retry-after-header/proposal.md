## Why

When the DriveThruRPG API returns HTTP 429 (Too Many Requests), it may include a `Retry-After` header telling the client exactly how long to wait before retrying. `LibraryClient::decode_response` currently discards all response headers once it reads the body, so `ClientError::ApiError` (the variant produced for any non-success status, including 429) carries only the numeric status code. Callers (the desktop app's retry logic) have no way to honor a server-specified wait and must fall back to blind exponential backoff, which risks retrying sooner than the server wants and prolonging the rate-limit window. This is the SDK-side half of a coordinated umbrella change (`dtrpg/openspec/changes/handle-http-429-retry-after`) — the app-side consumer cannot be built until this is exposed.

## What Changes

- `ClientError::ApiError` gains a `retry_after: Option<std::time::Duration>` field, populated from the response's `Retry-After` header when present on a non-success status.
- `decode_response` reads `response.headers()` for `Retry-After` before consuming the response body, parsing both the delay-in-seconds and HTTP-date forms permitted by RFC 9110 (falling back to `None` if the header is absent or unparseable in either form).
- No change to `ClientError::DecodeFailed` — that variant is only ever constructed on an already-successful status (decode failure after `status.is_success()`), where `Retry-After` is not a meaningful signal.

## Capabilities

### New Capabilities

- `rate-limit-retry-after`: The Rust SDK's library client captures and exposes the `Retry-After` response header on non-success HTTP responses so callers can honor server-specified retry delays instead of guessing.

### Modified Capabilities

_(none — `rust-library-client`'s existing requirements describe request construction and authentication, not error/retry semantics; this proposal doesn't change any of those requirements, only adds a new orthogonal one.)_

## Impact

- `src/library/client.rs`: `ClientError::ApiError` struct gains a field; `decode_response` reads and parses the `Retry-After` header before it's out of scope; `Display`/other `ClientError` impls updated for the new field.
- Downstream: `dtrpg-app/rust`'s SDK error-mapping layer (`crates/dtrpg-core/src/services/sdk/library/errors.rs`) will need a corresponding child change (tracked separately) to read this new field once this change ships.
