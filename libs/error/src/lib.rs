//! Error handling library - shared error types for all services
//!
//! This library provides core application error types that can be used across all services.
//! HTTP-specific features are available behind the `http` feature flag.
//!
//! ## Core Types
//!
//! - `AppError` - Standard application error enum for domain errors
//! - `ErrorContext` - Context information for error logging
//! - `ContextualError` - Error with context attached
//!
//! ## HTTP Features (behind `http` flag)
//!
//! - `ApiError` - Standardized API error response type
//! - `ApiResult` - Result type for API handlers
//! - `ErrorCode` - Standard HTTP error codes
//! - `AuthErrorCode` - Authentication/authorization specific error codes
//!
//! ## Usage
//!
//! ```rust
//! use error::{AppError, AppResult};
//!
//! fn example() -> AppResult {
//!     Err(AppError::not_found("user", "123"))
//! }
//! ```

// Declare modules
pub mod adapters;
pub mod core;

#[cfg(feature = "http")]
pub mod http;

pub use core::{AppError, AppResult};
