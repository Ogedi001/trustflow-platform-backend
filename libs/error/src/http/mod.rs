//! HTTP error handling module
//!
//! This module provides HTTP-level error types and utilities for consistent
//! API error responses across all services. This module is only available
//! when the `http` feature flag is enabled.
//!
//! ## Features
//!
//! - `ApiError` - Standardized API error response type
//! - `ApiResult` - Result type for API handlers
//! - `ErrorCode` - Standard HTTP error codes
//! - `AuthErrorCode` - Authentication/authorization specific error codes
//!
//! ## Usage
//!
//! ```rust
//! use error::http::{ApiError, ApiResult, ErrorCode};
//!
//! fn handler() -> ApiResult<User> {
//!     Err(ApiError::not_found("User not found"))
//! }
//! ```

pub mod api_error;
pub mod converters;
pub mod error_code;

pub use api_error::{ApiError, ApiResult, FieldError};
pub use error_code::ErrorCode;
// re-export domain auth codes for HTTP users
pub use crate::core::codes::auth_error::AuthErrorCode;
