## ADDED Requirements

### Requirement: Download preparation MUST include the target file's index
`prepare_download` MUST require the caller to supply the target file's `index` (its position within the ordered product's file list) as a request parameter, matching what the DriveThruRPG API enforces.

#### Scenario: Preparing a download with a valid index
- **WHEN** a caller invokes `prepare_download` with a valid `order_product_id` and the target file's `index`
- **THEN** the SDK sends the request with the index included and returns the deserialized response on success

#### Scenario: No default index is silently assumed
- **WHEN** a caller invokes `prepare_download`
- **THEN** the SDK's method signature requires an explicit index value — there is no method overload that omits it
