---
title: Authentication
version: latest
slug: authentication
---

# Authentication

`DriveThruRpgSdk` ties together your application configuration and session lifecycle. It
doesn't call the auth endpoint itself — give it an `AuthTokenResponse` from the
`/auth_key` call and it tracks the resulting `AuthSession`.

```rust
use dtrpg_sdk::{AuthTokenResponse, Config, DriveThruRpgSdk};

let mut sdk = DriveThruRpgSdk::with_config(Config::new("my-app-key"));

// After receiving an auth response from the API:
let response = AuthTokenResponse::new("jwt-token", "refresh-token", 1_800_000_000);
let session = sdk.apply_auth_response(response).unwrap();

assert_eq!(session.token(), "jwt-token");
assert_eq!(session.refresh_token(), "refresh-token");
```

## Session state

`AuthState` tracks whether the SDK is unconfigured, unauthenticated, authenticated, or the
token has expired. Calling `sdk.require_session()` before authenticating returns
`SdkError::Unauthenticated`; calling it before configuring returns `SdkError::Unconfigured`.

Invalidate a session (for example after the API reports an expired token) with
`sdk.invalidate_session(..)`, passing an `AuthSessionError` describing why:

```rust
use dtrpg_sdk::AuthState;
# use dtrpg_sdk::{AuthSessionError, AuthTokenResponse, Config, DriveThruRpgSdk};
# let mut sdk = DriveThruRpgSdk::with_config(Config::new("my-app-key"));
# sdk.apply_auth_response(AuthTokenResponse::new("jwt-token", "refresh-token", 1_800_000_000)).unwrap();

let error = AuthSessionError::new(
    "token_expired",
    "The authentication token has expired.",
    AuthState::TokenExpired,
);
sdk.invalidate_session(error).unwrap();
```

## Refreshing a session

An access token is short-lived. To renew it without repeating the application-key
exchange, call `refresh_session`. It sends the session's refresh token to the API and
replaces the stored session with the response, including the new refresh token and expiry.

```rust,no_run
# use dtrpg_sdk::{AuthTokenResponse, Config, DriveThruRpgSdk};
# async fn run() -> Result<(), dtrpg_sdk::ClientError> {
# let mut sdk = DriveThruRpgSdk::with_config(Config::new("my-app-key"));
# sdk.apply_auth_response(AuthTokenResponse::new("jwt-token", "refresh-token", 1_800_000_000)).unwrap();
sdk.refresh_session().await?;

// Clients created before the refresh still hold the old token.
let client = sdk.library_client().unwrap();
# Ok(())
# }
```

If the refresh token is rejected, the existing session is left in place. Fall back to
`key_exchange::authenticate` with the application key.

The refresh call is also available without a `DriveThruRpgSdk` as
`auth::token_refresh::refresh`.
