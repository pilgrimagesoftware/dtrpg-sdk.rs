## ADDED Requirements

### Requirement: ClientError::ApiError MUST expose the Retry-After header when present
When `LibraryClient` receives a non-success HTTP response, the resulting `ClientError::ApiError` SHALL include a `retry_after: Option<std::time::Duration>` field populated from the response's `Retry-After` header when that header is present and specifies a delay in seconds.

#### Scenario: 429 response with a delay-seconds Retry-After header
- **WHEN** the API returns HTTP 429 with a `Retry-After: 30` header
- **THEN** the returned `ClientError::ApiError`'s `retry_after` field is `Some(Duration::from_secs(30))`

#### Scenario: Non-success response with no Retry-After header
- **WHEN** the API returns a non-success status with no `Retry-After` header present
- **THEN** the returned `ClientError::ApiError`'s `retry_after` field is `None`

#### Scenario: Retry-After header present but unparseable as delay-seconds
- **WHEN** the API returns a non-success status with a `Retry-After` header that is not a valid non-negative integer (e.g. an HTTP-date value)
- **THEN** the returned `ClientError::ApiError`'s `retry_after` field is `None`, and no error is raised solely due to the unparseable header

#### Scenario: Success response is unaffected
- **WHEN** the API returns a success status
- **THEN** no `retry_after` value is computed or attached to any error (there is no error to attach it to), and response decoding proceeds as before
