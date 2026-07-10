## Why

`LibraryClient::create_product_list` decodes the `POST /{api_version}/product_lists`
response directly into `ProductListItem`, but the API wraps the resource in a JSON:API
`data` envelope: `{"data": {"id": "...", "type": "ProductList", "attributes": {...}}}`.
Decoding fails with `missing field 'id'` because the deserializer looks for `id` at the
payload's top level, which only holds a `data` key. This currently surfaces to the
`dtrpg-app` UI as an opaque decode-failure error whenever a user creates a collection from
the manage view.

## What Changes

- Add a `ProductListItemResponse { data: ProductListItem }` wrapper type in
  `library/models.rs`, matching the existing wrapper pattern used by
  `OrderProductItemResponse`.
- Change `create_product_list` to decode into `ProductListItemResponse` and return
  `response.data` as the `ProductListItem`, instead of decoding `ProductListItem` directly.
- Add a regression test (or extend existing decode tests) using a fixture payload shaped
  like the real API response (`data` envelope with `id`, `type`, `attributes`) to lock in
  the fix.

## Capabilities

### New Capabilities

_(none)_

### Modified Capabilities

- `rust-library-client`: `create_product_list` must decode the `data`-enveloped JSON:API
  response shape actually returned by `POST /{api_version}/product_lists`, rather than
  expecting the resource object at the payload's top level.

## Impact

- `dtrpg-sdk/rust/src/library/models.rs` — add `ProductListItemResponse`
- `dtrpg-sdk/rust/src/library/client.rs` — update `create_product_list` decode target
- `dtrpg-sdk/rust/src/library/models.rs` or `client.rs` tests — add regression coverage
