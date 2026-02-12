//! Converter for infrastructure errors to HTTP API errors

use crate::core::kinds::InfrastructureError;
use crate::http::ApiError;

impl From<InfrastructureError> for ApiError {
    fn from(error: InfrastructureError) -> Self {
        match error {
            InfrastructureError::Infrastructure { component, message } => {
                ApiError::service_unavailable(format!("{}: {}", component, message))
            }
            InfrastructureError::RateLimit {
                action,
                retry_after_seconds,
            } => ApiError::rate_limited_with_retry(action, retry_after_seconds),
        }
    }
}
