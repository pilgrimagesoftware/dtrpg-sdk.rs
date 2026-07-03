//! # DriveThruRPG SDK
//!
//! A Rust SDK for interacting with the [DriveThruRPG API](https://api.drivethrurpg.com).
//!
//! ## Overview
//!
//! This crate provides types and structures for authenticating with, configuring, and
//! making requests to the DriveThruRPG API. It covers:
//!
//! - **Configuration** — supplying your application key, API base URL, and API version
//!   via [`Config`].
//! - **Authentication** — representing token responses, active sessions, and session state
//!   via [`AuthTokenResponse`], [`AuthSession`], and [`AuthState`].
//! - **Error handling** — structured errors for SDK-level, session-level, and HTTP failures
//!   via [`SdkError`], [`AuthSessionError`], and [`ClientError`].
//! - **SDK entry point** — [`DriveThruRpgSdk`] ties configuration and session lifecycle
//!   together and vends a [`LibraryClient`] once authenticated.
//! - **Library access** — [`LibraryClient`] provides an async HTTP client for all library
//!   endpoints (ordered products, product lists, download preparation).
//! - **Library types** — Rust model types for every API-defined library schema, such as
//!   [`OrderProductItem`], [`ProductListItem`], and their supporting structures.
//!
//! ## Quick Start
//!
//! ```rust
//! use dtrpg_sdk::{Config, DriveThruRpgSdk, AuthTokenResponse};
//!
//! let mut sdk = DriveThruRpgSdk::with_config(Config::new("my-app-key"));
//!
//! // After receiving an auth response from the API:
//! let response = AuthTokenResponse::new("jwt-token", "refresh-token", 1_800_000_000);
//! let session = sdk.apply_auth_response(response).unwrap();
//! assert_eq!(session.token(), "jwt-token");
//!
//! // Create an authenticated library client:
//! let client = sdk.library_client().unwrap();
//! // client.list_order_products(Default::default()).await ...
//! ```

pub mod auth;
pub mod auth_client;
pub mod client;
pub mod config;
pub mod error;
pub mod library;
pub mod openapi;
pub mod sdk;

pub use auth::{AuthSession, AuthState, AuthTokenResponse, SessionTransition};
pub use client::{ClientError, LibraryClient};
pub use config::Config;
pub use error::{AuthSessionError, SdkError};
pub use library::{
    FileChecksum, IncludedItem, LibraryItemsParams, OrderProductAttribute,
    OrderProductAttributes, OrderProductDescription, OrderProductFile, OrderProductFilter,
    OrderProductHistoryEntry, OrderProductInfo, OrderProductItem, OrderProductItemResponse,
    OrderProductListResponse, OrderProductOrder, OrderProductPublisher,
    OrderProductRelationships, PageParams, PaginationLinks, PaginationMeta, ProductListAttributes,
    ProductListCollectionResponse, ProductListItem, ProductListItemsResponse, PublisherAttributes,
    PublisherItem, RelationshipData, RelationshipRef,
};
pub use openapi::{OPERATIONS, OpenApiOperation};
pub use sdk::DriveThruRpgSdk;

#[cfg(test)]
mod tests {
    use super::{
        AuthSessionError, AuthState, AuthTokenResponse, Config, DriveThruRpgSdk, SdkError,
    };

    #[test]
    fn sdk_requires_configuration_before_auth_session_is_applied() {
        let mut sdk = DriveThruRpgSdk::new();
        let response = AuthTokenResponse::new("jwt-token", "refresh-token", 1_771_547_233);

        let error = sdk.apply_auth_response(response).unwrap_err();

        assert_eq!(error, SdkError::Unconfigured);
    }

    #[test]
    fn sdk_stores_api_defined_auth_session_after_configuration() {
        let mut sdk = DriveThruRpgSdk::with_config(Config::new("app-key"));
        let response = AuthTokenResponse::new("jwt-token", "refresh-token", 1_771_547_233);

        let session = sdk.apply_auth_response(response).unwrap();

        assert_eq!(session.token(), "jwt-token");
        assert_eq!(session.refresh_token(), "refresh-token");
        assert!(!session.refresh_token_expired_at(1_771_547_232));
        assert!(session.refresh_token_expired_at(1_771_547_233));
    }

    #[test]
    fn invalidating_session_preserves_api_auth_state_meaning() {
        let mut sdk = DriveThruRpgSdk::with_config(Config::new("app-key"));
        let response = AuthTokenResponse::new("jwt-token", "refresh-token", 1_771_547_233);
        sdk.apply_auth_response(response).unwrap();

        let error = AuthSessionError::new(
            "token_expired",
            "The authentication token has expired.",
            AuthState::TokenExpired,
        );

        let invalidation = sdk.invalidate_session(error.clone()).unwrap();

        assert_eq!(invalidation, error);
        assert_eq!(
            sdk.require_session().unwrap_err(),
            SdkError::Unauthenticated
        );
    }

    #[test]
    fn rust_sdk_generates_api_metadata_from_openapi_spec() {
        assert_eq!(
            crate::openapi::DEFAULT_SERVER_URL,
            "https://api.drivethrurpg.com/api"
        );
        assert!(crate::openapi::OPENAPI_SPEC_BYTES > 0);
        assert!(
            crate::openapi::OPERATIONS.contains(&crate::OpenApiOperation {
                method: "POST",
                path: "/{DTRPG_API_VERSION}/auth_key",
            })
        );
        assert!(
            crate::openapi::OPERATIONS.contains(&crate::OpenApiOperation {
                method: "GET",
                path: "/{DTRPG_API_VERSION}/order_products",
            })
        );
    }
}
