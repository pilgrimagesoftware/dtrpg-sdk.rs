## Context

`prepare_download` (`src/library/client.rs`) currently sends `GET /{api_version}/order_products/{order_product_id}/prepare` with no query parameters. Verified against a live account with a real `order_product_id`: the response is HTTP 403 with body `"Unable to prepare a file without an index"` regardless of which order product is requested. Adding `?index=0` (and `?index=1` for a second file in the same bundle) both returned HTTP 200 with a full `WatermarkTaskStatus` payload.

The `index` value matches `OrderProductFile.index` (`src/library/models.rs`), already deserialized from other endpoints (e.g. the ordered-product detail response) — this change only adds a way to pass that already-known value into `prepare_download`, it doesn't introduce a new concept to the SDK.

## Goals / Non-Goals

**Goals:**
- `prepare_download` succeeds against the real API for a valid `(order_product_id, index)` pair.
- Keep the change minimal: fix the broken request, don't attempt to also formalize the response type in the same change.

**Non-Goals:**
- No typed response model for the `WatermarkTaskStatus` payload — the response is still `serde_json::Value`, matching the existing documented rationale ("the schema for this endpoint has not yet been formally defined by the API contract"). Formalizing it (including the `status`/`progress` polling semantics observed live) is a larger, separate effort once more of the shape is understood across different file types/states.
- No retry/polling logic for a `status: "Processing"` response — this change only fixes request construction; handling an in-progress watermark job is a caller-side (or future SDK-side) concern.

## Decisions

### Add `index: u32` as a required second parameter, not optional

The API rejects the request outright without it — there is no valid default (`index: 0` would silently prepare the wrong file for any multi-file bundle where the caller wants a later file), so making it required forces every call site to supply a real value rather than accidentally relying on a default that's only sometimes correct. This is a breaking signature change; the proposal marks it as such.

_Alternative considered_: A separate `prepare_download_indexed` method, leaving `prepare_download(order_product_id)` in place defaulting to index 0. Rejected — a same-named method with a silent wrong-file default is worse than a compiler-enforced breaking change; every existing caller needs to think about which index it means anyway.

## Risks / Trade-offs

- [Risk] Breaking change requires a version bump and every downstream caller (currently only `dtrpg-app/rust`, mid-implementation of `implement-real-file-downloads`) to update in lockstep. → Mitigation: coordinate the SDK release with that change's implementation; `dtrpg-app`'s change is already blocked on this landing, so there's no independent caller to break unexpectedly.
