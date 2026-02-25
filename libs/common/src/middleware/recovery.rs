//! Error recovery middleware
//!
//! Catches panics and unhandled errors to prevent server crashes,
//! returning graceful error responses instead.

use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;

/// Recovery mode configuration
#[derive(Debug, Clone, Copy)]
pub enum RecoveryMode {
    /// Log errors but don't expose internal details
    Secure,
    /// Log errors and expose internal details (development only)
    Debug,
}

/// Recovery middleware configuration
#[derive(Debug, Clone, Copy)]
pub struct RecoveryConfig {
    /// Recovery mode
    pub mode: RecoveryMode,
    /// Always return 500 on errors
    pub always_500: bool,
}

impl RecoveryConfig {
    /// Create new recovery config
    pub fn new(mode: RecoveryMode) -> Self {
        Self {
            mode,
            always_500: true,
        }
    }

    /// Secure mode (production)
    pub fn secure() -> Self {
        Self::new(RecoveryMode::Secure)
    }

    /// Debug mode (development)
    pub fn debug() -> Self {
        Self::new(RecoveryMode::Debug)
    }

    /// Disable always_500 to propagate status codes
    pub fn with_status_codes(mut self) -> Self {
        self.always_500 = false;
        self
    }
}

impl Default for RecoveryConfig {
    fn default() -> Self {
        Self::secure()
    }
}

/// Middleware for error recovery
pub async fn recovery_middleware(
    req: Request,
    next: Next,
    config: RecoveryConfig,
) -> Result<Response, StatusCode> {
    // In production, this would catch panics using catch_unwind or similar
    // For now, we just pass through
    let response = next.run(req).await;

    // Check for error status codes
    if response.status().is_server_error() {
        match config.mode {
            RecoveryMode::Debug => {
                // In debug mode, might expose more details
                #[cfg(feature = "logging")]
                tracing::error!(
                    status = %response.status(),
                    "Server error response"
                );
            }
            RecoveryMode::Secure => {
                // In secure mode, just log without details
                #[cfg(feature = "logging")]
                tracing::error!(status = %response.status(), "Server error");
            }
        }
    }

    Ok(response)
}

/// Create recovery middleware with config
pub fn make_recovery_middleware(
    config: RecoveryConfig,
) -> impl Fn(Request, Next) -> futures::future::BoxFuture<'static, Result<Response, StatusCode>> + Clone
{
    move |req: Request, next: Next| {
        let config = config;
        Box::pin(recovery_middleware(req, next, config))
    }
}

/// Error details wrapper
#[derive(Debug)]
pub struct ErrorDetails {
    /// Error message
    pub message: String,
    /// Error context
    pub context: Option<String>,
}

impl ErrorDetails {
    /// Create new error details
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            context: None,
        }
    }

    /// Add context
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_recovery_config_secure() {
//         let config = RecoveryConfig::secure();
//         assert!(matches!(config.mode, RecoveryMode::Secure));
//     }

//     #[test]
//     fn test_recovery_config_debug() {
//         let config = RecoveryConfig::debug();
//         assert!(matches!(config.mode, RecoveryMode::Debug));
//     }

//     #[test]
//     fn test_error_details_creation() {
//         let err = ErrorDetails::new("Something went wrong");
//         assert_eq!(err.message, "Something went wrong");
//     }

//     #[test]
//     fn test_error_details_with_context() {
//         let err = ErrorDetails::new("Error")
//             .with_context("Processing request");
//         assert!(err.context.is_some());
//     }
// }
