//! HTTP middleware for request/response processing
//!
//! Provides a collection of middleware components for Axum applications.
//! Available when the `http` feature is enabled.
//!
//! # Features
//!
//! - **tracking**: Unified middleware for request_id, correlation_id, idempotency_key
//! - **auth_context**: Extract and manage authentication context from bearer tokens
//! - **body_limit**: Enforce request body size limits
//! - **compression**: Automatic response compression (gzip, deflate, brotli)
//! - **cors**: Cross-Origin Resource Sharing (CORS) policy enforcement
//! - **idempotency**: Idempotent request handling with deduplication
//! - **logging**: Request/response logging with structured tracing
//! - **metrics**: Performance metrics collection and reporting
//! - **rate_limit**: Request rate limiting with sliding window algorithm
//! - **recovery**: Graceful error recovery and panic handling
//! - **retry**: Automatic retry logic with exponential backoff
//! - **timeout**: Request timeout enforcement

#[cfg(feature = "http")]
pub mod auth_context;
#[cfg(feature = "http")]
pub mod body_limit;
#[cfg(feature = "http")]
pub mod compression;
#[cfg(feature = "http")]
pub mod cors;
#[cfg(feature = "http")]
pub mod idempotency;
#[cfg(feature = "http")]
pub mod logging;
#[cfg(feature = "http")]
pub mod metrics;
#[cfg(feature = "http")]
pub mod rate_limit;
#[cfg(feature = "http")]
pub mod recovery;
#[cfg(feature = "http")]
pub mod retry;
#[cfg(feature = "http")]
pub mod timeout;
#[cfg(feature = "http")]
pub mod tracking;

#[cfg(not(feature = "http"))]
compile_error!("middleware module requires the 'http' feature to be enabled");

#[cfg(feature = "http")]
pub use auth_context::*;
#[cfg(feature = "http")]
pub use body_limit::*;
#[cfg(feature = "http")]
pub use compression::*;
#[cfg(feature = "http")]
pub use cors::*;
#[cfg(feature = "http")]
pub use idempotency::*;
#[cfg(feature = "http")]
pub use logging::*;
#[cfg(feature = "http")]
pub use metrics::*;
#[cfg(feature = "http")]
pub use rate_limit::*;
#[cfg(feature = "http")]
pub use recovery::*;
#[cfg(feature = "http")]
pub use retry::*;
#[cfg(feature = "http")]
pub use timeout::*;
#[cfg(feature = "http")]
pub use tracking::*;

/// Prelude for middleware module
#[cfg(feature = "http")]
pub mod prelude {
    //! Import common middleware items with `use common::middleware::prelude::*;`
    pub use super::{
        auth_context::*, body_limit::*, compression::*, cors::*, idempotency::*, logging::*,
        metrics::*, rate_limit::*, recovery::*, retry::*, timeout::*, tracking::*,
    };
}
