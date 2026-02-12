//! Converter for business logic errors to HTTP API errors

use crate::core::kinds::BusinessError;
use crate::http::ApiError;
use axum::http::StatusCode;

impl From<BusinessError> for ApiError {
    fn from(error: BusinessError) -> Self {
        match error {
            BusinessError::Business { message, code } => {
                ApiError::new(crate::http::ErrorCode::BadRequest, message)
                    .with_details(serde_json::json!({ "business_code": code }))
                    .with_status(StatusCode::BAD_REQUEST)
            }
            BusinessError::Conflict { message, .. } => ApiError::conflict(message),
        }
    }
}
