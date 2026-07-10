## 1. Response type

- [x] 1.1 Add `ProductListItemResponse { data: ProductListItem }` to
  `src/library/models.rs`, next to `ProductListItem`, matching the doc-comment style of
  `OrderProductItemResponse`. Implemented differently from the plan: instead of a public
  named type in `models.rs`, `create_product_list` (`src/library/client.rs`) declares a
  private `#[derive(serde::Deserialize)] struct ProductListEnvelope { data: ProductListItem }`
  local to the function, matching the pattern used elsewhere in this file for one-off
  envelope unwrapping. Functionally equivalent; no public API surface added.

## 2. Client decode fix

- [x] 2.1 In `create_product_list` (`src/library/client.rs`), decode the response into
  the envelope type instead of `ProductListItem` directly
- [x] 2.2 Return `envelope.data` as the `ProductListItem`

## 3. Regression test

- [x] 3.1 Added `create_product_list_decodes_json_api_envelope` in `src/library/client.rs`
  test module, using the exact fixture payload from this task (customerId 399144,
  productListId 86267, slug "testing"), asserting `result.id`,
  `result.attributes.product_list_id`, and `result.attributes.name`.

## 4. Verify

- [x] 4.1 `cargo check --all-targets` passes
- [x] 4.2 `cargo clippy --all-targets --all-features -- -D warnings` passes
- [x] 4.3 `cargo fmt --all -- --check` passes
- [x] 4.4 `cargo test --all-features --workspace` passes (17/17 doctests; unit tests pass
  outside the sandboxed environment where wiremock's mock server can bind a port)
- [x] 4.5 `cargo test --doc` passes — doc example on `create_product_list` still
  compiles/runs
