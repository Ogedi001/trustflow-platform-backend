//! Core application error type
//!
//! This module contains the main AppError enum that wraps all domain error kinds.

use super::kinds::*;
use crate::core::codes::auth_error::AuthErrorCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
/// Standard application error type
///
/// This enum represents the core domain errors that can occur in the application.
/// Each variant represents a category of errors that can happen during operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
#[serde(tag = "error_type", content = "message")]
pub enum AppError {
    /// Validation errors (invalid input, missing fields)
    #[error("Validation error: {0}")]
    ValidationError(ValidationError),

    /// Authentication errors
    #[error("Authentication failed: {0}")]
    AuthenticationError(AuthError),

    /// Authorization errors (permission denied)
    #[error("Authorization failed: {0}")]
    AuthorizationError(AuthError),

    /// Not found errors
    #[error("{0}")]
    NotFoundError(NotFoundError),

    /// Conflict errors (duplicate, already exists)
    #[error("Conflict: {0}")]
    ConflictError(BusinessError),

    /// Rate limit errors
    #[error("Rate limit exceeded: {0}")]
    RateLimitError(InfrastructureError),

    /// Business logic errors
    #[error("Business error: {0}")]
    BusinessError(BusinessError),

    /// External service errors
    #[error("External service error: {0}")]
    ExternalServiceError(ExternalServiceError),

    /// Database errors
    #[error("Database error: {0}")]
    DatabaseError(DatabaseError),

    /// Infrastructure errors (cache, queue, storage)
    #[error("Infrastructure error: {0}")]
    InfrastructureError(InfrastructureError),

    /// Unknown/internal errors
    #[error("Internal error: {0}")]
    InternalError(InternalError),
}

/// Result type alias for operations returning `AppError`
pub type AppResult<T = ()> = Result<T, AppError>;

/// Helper functions for creating common errors
impl AppError {
    /// Create a validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Self::ValidationError(ValidationError::new(message))
    }

    /// Create a validation error with field
    pub fn validation_with_field(message: impl Into<String>, field: impl Into<String>) -> Self {
        Self::ValidationError(ValidationError::with_field(message, field))
    }

    /// Create an authentication error
    pub fn auth(message: impl Into<String>, code: AuthErrorCode) -> Self {
        Self::AuthenticationError(AuthError::auth(message, code))
    }

    /// Create an authorization error
    pub fn authz(message: impl Into<String>, code: AuthErrorCode) -> Self {
        Self::AuthorizationError(AuthError::authz(message, code))
    }

    /// Create a not found error
    pub fn not_found(resource: impl Into<String>, id: impl Into<String>) -> Self {
        Self::NotFoundError(NotFoundError::new(resource, id))
    }

    /// Create a conflict error
    pub fn conflict(message: impl Into<String>) -> Self {
        Self::ConflictError(BusinessError::conflict(message))
    }

    /// Create a rate limit error
    pub fn rate_limit(action: impl Into<String>, retry_after: u64) -> Self {
        Self::RateLimitError(InfrastructureError::rate_limit(action, retry_after))
    }

    /// Create a business error
    pub fn business(message: impl Into<String>, code: impl Into<String>) -> Self {
        Self::BusinessError(BusinessError::business(message, code))
    }

    /// Create an external service error
    pub fn external(service: impl Into<String>, message: impl Into<String>) -> Self {
        Self::ExternalServiceError(ExternalServiceError::new(service, message))
    }

    /// Create a database error
    pub fn database(message: impl Into<String>) -> Self {
        Self::DatabaseError(DatabaseError::new(message))
    }

    /// Create an infrastructure error
    pub fn infrastructure(component: impl Into<String>, message: impl Into<String>) -> Self {
        Self::InfrastructureError(InfrastructureError::infrastructure(component, message))
    }

    /// Create an internal error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::InternalError(InternalError::new(message))
    }

    /// Create an internal error with a source error
    pub fn internal_with_source(message: impl Into<String>, source: impl Into<String>) -> Self {
        let source_msg = source.into();
        Self::InternalError(InternalError::with_source(
            message,
            Arc::new(std::io::Error::new(std::io::ErrorKind::Other, source_msg)),
        ))
    }

    /// Create a bad request error (400)
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::validation(message)
    }

    /// Create a rate limited error (429)
    pub fn rate_limited(message: impl Into<String>, retry_after: u64) -> Self {
        Self::rate_limit(message, retry_after)
    }

    /// Wrap an error
    pub fn with_source(self, source: Arc<dyn std::error::Error + Send + Sync>) -> Self {
        match self {
            Self::InternalError(mut internal) => {
                internal.source = Some(source);
                Self::InternalError(internal)
            }
            _ => self,
        }
    }
}

// Core From implementations that don't require HTTP features
// These are safe to implement here as they don't depend on web frameworks

/// Convert from std::io::Error
impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        Self::infrastructure("io", e.to_string())
    }
}

/// Convert from base64::DecodeError
impl From<base64::DecodeError> for AppError {
    fn from(e: base64::DecodeError) -> Self {
        Self::validation(format!("Invalid base64: {}", e))
    }
}

/// Convert from uuid::Error
impl From<uuid::Error> for AppError {
    fn from(e: uuid::Error) -> Self {
        Self::validation(format!("Invalid UUID: {}", e))
    }
}

/// Convert from time::error::Parse
impl From<time::error::Parse> for AppError {
    fn from(e: time::error::Parse) -> Self {
        Self::validation(format!("Invalid date/time format: {}", e))
    }
}
