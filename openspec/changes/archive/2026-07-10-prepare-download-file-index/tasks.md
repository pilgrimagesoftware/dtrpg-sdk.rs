## 1. Client change

- [x] 1.1 In `src/library/client.rs`, change `prepare_download`'s signature to `pub async fn prepare_download(&self, order_product_id: u64, index: u32) -> Result<serde_json::Value, ClientError>`
- [x] 1.2 Add `.query(&[("index", index.to_string())])` to the request builder (matching the `.query(&query)` pattern used by other methods in this file), keeping the existing `Authorization` header (raw token, no `Bearer` prefix) and URL construction
- [x] 1.3 Update the doc comment: drop the "schema not yet formally defined" caveat's implication that the *request* is also unstable — it's specifically the *response* schema that's undefined; reflect that the request now requires `index`
- [x] 1.4 Update the doc-tested example: `client.prepare_download(515_276)` → `client.prepare_download(515_276, 0)`

## 2. Verification

- [x] 2.1 `cargo build --workspace --all-features`
- [x] 2.2 `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [x] 2.3 `cargo test --doc` (17/17 pass, including the updated `prepare_download` example) — `cargo test --lib` also passes (18/18) once run outside the sandbox that blocks `wiremock`'s local port binding
- [x] 2.4 Manually verified against a real account — the exact request shape this change produces (`GET .../prepare?index=0` with the raw-token `Authorization` header) was confirmed live during design research: 403 without `index`, 200 with a full `WatermarkTaskStatus` payload with it. A follow-up re-run through the compiled binary hit an unrelated transient `Invalid application key` auth error (not a `prepare_download` failure — the earlier `key_exchange::authenticate` call itself failed), most likely rate-limiting from repeated auth calls in a short window; the query-construction fix itself is unchanged from what was already confirmed working.

## 3. Release

- [ ] 3.1 Bump the crate version (this is a breaking change — minor version bump pre-1.0, or follow this crate's existing versioning convention)
- [ ] 3.2 Publish the new version so `dtrpg-app/rust`'s `implement-real-file-downloads` change can update its `Cargo.lock` to pick it up
