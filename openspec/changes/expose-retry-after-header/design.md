## Context

`decode_response` (`src/library/client.rs`) is the single choke point every `LibraryClient` method routes through for turning a `reqwest::Response` into either a decoded value or a `ClientError`. On `!status.is_success()` it currently reads only the status code and body before returning `ClientError::ApiError { url, status, message, payload }` — `response.headers()` is never inspected. `reqwest::Response::bytes()` consumes the response, so headers must be read before that call.

Per [RFC 9110 §10.2.3](https://www.rfc-editor.org/rfc/rfc9110#field.retry-after), `Retry-After` may be either `delay-seconds` (an integer) or an HTTP-date. No `httpdate`/`chrono`-equivalent date-parsing crate is currently a dependency of this SDK.

## Goals / Non-Goals

**Goals:**
- Capture `Retry-After` on any non-success response and expose it as a typed `Duration` via `ClientError::ApiError`.
- Fail safe: an absent or unparseable header yields `None`, never a panic or a different error variant.

**Non-Goals:**
- Parsing the HTTP-date form of `Retry-After`. Delay-seconds is both the simpler form and, in practice, what rate-limit responses use (an HTTP-date requires the server to format a clock-synced absolute time, which is unusual for a `429` specifically — it's more common on redirect/maintenance responses). Supporting it would require adding a date-parsing dependency for a form this API is unlikely to send; if a future observation shows DriveThruRPG does send the date form, that's a follow-up change with its own justification for the added dependency.
- Changing `ClientError::DecodeFailed`, `Http`, or any other variant — only `ApiError` is affected, since it's the only variant produced for a non-success status where a retry decision is meaningful.
- Any retry-loop or backoff logic — that's the app child change's scope (`dtrpg-app/rust/openspec/changes/handle-http-429-retry-after`).

## Decisions

**Add `retry_after: Option<std::time::Duration>` to `ClientError::ApiError`**, not a new top-level variant. `ApiError` already carries `status: u16`, so a 429 is already representable there; adding a field is a smaller, additive change than introducing `ClientError::RateLimited { .. }` as a parallel variant, and keeps every existing `ApiError` match arm in downstream code compiling (with a new field to destructure or ignore via `..`) rather than requiring a new arm everywhere `ClientError` is matched.

**Parse only the delay-seconds form**, via `header_value.to_str().ok().and_then(|s| s.trim().parse::<u64>().ok()).map(Duration::from_secs)`. No new dependency needed — `u64::from_str` covers the entire grammar for `delay-seconds`.

**Read the header before calling `response.bytes()`.** `reqwest::Response::headers()` borrows the response and is available before the body is consumed; `retry_after` is computed first, stored in a local, and moved into the `ApiError` construction alongside the already-read `status`/`payload`/`message`.

## Risks / Trade-offs

- [DriveThruRPG sends the HTTP-date form on some future rate-limit response] → `retry_after` is `None` in that case, same as today's behavior (no `Retry-After` support at all) — a soft regression-free gap, not a new failure mode. The app's fallback (existing exponential backoff) still applies.
- [A caller pattern-matches `ClientError::ApiError { url, status, message, payload }` exhaustively without `..`] → Existing call sites in this crate and in `dtrpg-app/rust` were checked; none destructure `ApiError` exhaustively without a trailing binding that would need updating anyway for a new field (grep confirms downstream matches use field-subset patterns already, e.g. `ClientError::ApiError { status: 404, .. }`), so this is not expected to be a breaking change in practice, but callers should confirm with their own build after upgrading.
