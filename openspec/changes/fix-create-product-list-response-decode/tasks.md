## 1. Response type

- [ ] 1.1 Add `ProductListItemResponse { data: ProductListItem }` to
  `src/library/models.rs`, next to `ProductListItem`, matching the doc-comment style of
  `OrderProductItemResponse`

## 2. Client decode fix

- [ ] 2.1 In `create_product_list` (`src/library/client.rs`), decode the response into
  `ProductListItemResponse` instead of `ProductListItem`
- [ ] 2.2 Return `response.data` as the `ProductListItem`

## 3. Regression test

- [ ] 3.1 Add a unit test (in `src/library/models.rs` or `src/library/client.rs`, following
  existing test placement conventions in this crate) that deserializes a fixture payload
  shaped like the real API response — `{"data":{"id":"/api/vBeta/product_lists/86267","type":"ProductList","attributes":{"customerId":399144,"name":"Testing","dateCreated":"2026-07-09T00:42:39-05:00","productListId":86267,"slug":"testing","itemCount":0}}}`
  — into `ProductListItemResponse` and asserts the unwrapped fields

## 4. Verify

- [ ] 4.1 Run `cargo check --all-targets`
- [ ] 4.2 Run `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] 4.3 Run `cargo fmt --all -- --check`
- [ ] 4.4 Run `cargo test --all-features --workspace`
- [ ] 4.5 Run `cargo test --doc` (doc example on `create_product_list` still compiles/runs)
