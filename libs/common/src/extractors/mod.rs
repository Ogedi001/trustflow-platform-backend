//! HTTP request extractors (Axum-based)
//!
//! Provides custom request extractors for common patterns.

#[cfg(feature = "http")]
pub mod json;

#[cfg(feature = "http")]
pub mod query;

#[cfg(feature = "http")]
pub mod path;

#[cfg(feature = "http")]
pub mod auth;

#[cfg(feature = "http")]
pub use auth::*;

#[cfg(feature = "http")]
pub use json::*;

#[cfg(feature = "http")]
pub use path::*;

#[cfg(feature = "http")]
pub use query::*;

#[cfg(not(feature = "http"))]
compile_error!("extractors module requires 'http' feature to be enabled");
