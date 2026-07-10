## Why

`LibraryClient::prepare_download(order_product_id)` calls `GET /order_products/{id}/prepare` with no way to specify which file to prepare. Verified against a live account: the endpoint returns HTTP 403 ("Unable to prepare a file without an index") unless an `index` query parameter is supplied — the SDK's own `OrderProductFile` model already carries this same `index: u32` field, so the data is available, just not threaded through this call. This blocks `dtrpg-app`'s `implement-real-file-downloads` change, which needs `prepare_download` to actually work.

## What Changes

- `prepare_download` gains a required `index: u32` parameter: `prepare_download(&self, order_product_id: u64, index: u32) -> Result<serde_json::Value, ClientError>`. **BREAKING**: changes the method's arity; all callers must be updated.
- No change to the response type (still `serde_json::Value` — the response schema itself is a separate, larger formalization effort; this change only fixes the request side, which is what's currently broken).

## Capabilities

### Modified Capabilities

- `rust-library-client`: the download-preparation capability ("MUST provide an async HTTP client for all library endpoints", specifically download preparation) now requires callers to supply the target file's index, matching what the live API actually enforces.

## Impact

- `src/library/client.rs`: `prepare_download`'s signature and the request URL it builds.
- Doc examples in the same file's `prepare_download` doc comment (`client.prepare_download(515_276)` becomes `client.prepare_download(515_276, 0)`).
- Downstream: `dtrpg-app/rust`'s `implement-real-file-downloads` change depends on this landing (and on a new `dtrpg-sdk` release being picked up by `dtrpg-app`'s `Cargo.lock`) before it can call `prepare_download` correctly.
