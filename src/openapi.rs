//! Build-generated OpenAPI metadata.
//!
//! This module is generated from `API/openapi.yaml` at compile time. It gives the
//! Rust crate a direct dependency on the same API contract file used by the other
//! SDKs, while leaving room to expand into generated request/response clients.

include!(concat!(env!("OUT_DIR"), "/openapi.rs"));
