//! Converter for external service errors to HTTP API errors

use crate::core::kinds::ExternalServiceError;
use crate::http::ApiError;
use axum::http::StatusCode;

impl From<ExternalServiceError> for ApiError {
    fn from(error: ExternalServiceError) -> Self {
        let status = error
            .status_code
            .and_then(|code| StatusCode::from_u16(code).ok())
            .unwrap_or(StatusCode::BAD_GATEWAY);

        ApiError::new(
            crate::http::ErrorCode::BadGateway,
            format!("{} - {}", error.service, error.message),
        )
        .with_status(status)
    }
}
