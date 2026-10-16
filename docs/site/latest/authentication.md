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
