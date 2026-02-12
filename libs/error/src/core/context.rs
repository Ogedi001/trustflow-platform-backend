//! Error context for logging and debugging
//!
//! This module provides additional context information that can be attached
//! to errors for better debugging and logging.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Error context for logging
///
/// This struct provides additional context information that can be attached
/// to errors for better debugging and logging.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ErrorContext {
    /// Request ID for tracing
    pub request_id: Option<String>,
    /// User ID for user-specific errors
    pub user_id: Option<String>,
    /// Resource identifier
    pub resource: Option<String>,
    /// Action that triggered the error
    pub action: Option<String>,
    /// Additional metadata as JSON
    pub metadata: Option<serde_json::Value>,
}

/// Error with context
///
/// This type wraps an `AppError` with additional context information
/// for logging and debugging purposes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextualError {
    /// The underlying error
    pub error: crate::core::AppError,
    /// Error context information
    pub context: ErrorContext,
    /// Timestamp when the error occurred
    pub timestamp: time::OffsetDateTime,
}

impl ContextualError {
    /// Create a new contextual error from an AppError
    pub fn new(error: crate::core::AppError) -> Self {
        Self {
            error,
            context: ErrorContext::default(),
            timestamp: time::OffsetDateTime::now_utc(),
        }
    }

    /// Add context to the error
    pub fn with_context(mut self, context: ErrorContext) -> Self {
        self.context = context;
        self
    }

    /// Add request ID to the error context
    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.context.request_id = Some(request_id.into());
        self
    }

    /// Add user ID to the error context
    pub fn with_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.context.user_id = Some(user_id.into());
        self
    }
}

impl fmt::Display for ContextualError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.timestamp, self.error)
    }
}

impl std::error::Error for ContextualError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
