## Context

`LibraryClient::create_product_list` sends `POST /{api_version}/product_lists` and passes
the raw `Response` to `decode_response::<ProductListItem>`. The real API wraps the created
resource in a JSON:API `data` envelope (confirmed from a live payload):

```json
{"data":{"id":"/api/vBeta/product_lists/86267","type":"ProductList","attributes":{"customerId":399144,"name":"Testing","dateCreated":"2026-07-09T00:42:39-05:00","productListId":86267,"slug":"testing","itemCount":0}}}
```

`ProductListItem` expects `id`, `type`, and `attributes` at the top level, so deserialization
fails with `missing field 'id'`. This is the same envelope shape `OrderProductItemResponse`
already handles for the single-order-product endpoint, so the fix follows that existing
pattern rather than inventing a new one.

## Goals / Non-Goals

**Goals:**
- Make `create_product_list` decode the actual response shape returned by the API.
- Follow the existing `data`-wrapper convention already used by
  `OrderProductItemResponse` for consistency.
- Add regression coverage using a fixture shaped like the real payload above.

**Non-Goals:**
- Auditing every other client method for the same class of bug (out of scope for this fix;
  only `create_product_list` has been confirmed broken by a live payload).
- Changing the `ProductListItem` or `ProductListAttributes` field shapes.

## Decisions

- **Add `ProductListItemResponse { data: ProductListItem }` and decode into it, then return
  `.data`** rather than making `decode_response` envelope-aware generically. Other single-
  resource responses in this module already use a dedicated wrapper type
  (`OrderProductItemResponse`) rather than a generic `Envelope<T>`, so this keeps the new
  code consistent with the surrounding style instead of introducing a second convention.

## Risks / Trade-offs

- [Other create/update endpoints may have the same unenveloped-decode bug but haven't
  produced a live failure yet] → Out of scope here; worth a follow-up audit of
  `decode_response` call sites once this fix lands.

## Migration Plan

No migration needed — this is an internal decode-target fix with no change to the public
`create_product_list` signature or return type.
