//! HTTP API error types for consistent error responses
//!
//! Provides ApiError and ApiResult types for standardized HTTP API responses
//! across all services.

use axum::{
    Json,
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde::Serialize;
use serde_json::Value;

use crate::core::AuthErrorCode;

use super::error_code::ErrorCode;

/// Represents a field-level validation error
#[derive(Debug, Serialize, Clone)]
pub struct FieldError {
    /// The name of the field that has the error
    pub field: String,

    /// Human-readable error message for this field
    pub message: String,
}

/// API error response type
///
/// This type represents a standardized API error response that includes:
/// - A machine-readable error code
/// - A human-readable error message
/// - Optional details (validation errors, field errors, etc.)
/// - HTTP status code (for internal use, not serialized)
#[derive(Debug, Serialize, Clone)]
#[serde(tag = "error_type")]
pub struct ApiError {
    /// Machine-readable error code (e.g., "VALIDATION_ERROR", "NOT_FOUND")
    pub code: ErrorCode,

    /// Human-readable error message (safe to display to users)
    pub message: String,

    /// Additional error details (field-specific errors, validation details, etc.)
    ///
    /// This can contain:
    /// - List of field errors for validation failures
    /// - Detailed error information for debugging
    /// - Any supplementary data relevant to the error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,

    /// HTTP status code (not serialized to JSON, used for HTTP response)
    #[serde(skip)]
    pub status_code: Option<StatusCode>,
}

impl ApiError {
    /// Create a new API error with just code and message
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            details: None,
            status_code: None,
        }
    }

    /// Create with custom error code (for auth-specific codes)
    pub fn with_code(code: impl Into<ErrorCode>, message: impl Into<String>) -> Self {
        Self::new(code.into(), message)
    }

    /// Add additional details to the error
    pub fn with_details<T: Serialize>(mut self, details: T) -> Self {
        self.details = Some(serde_json::to_value(details).unwrap_or(Value::Null));
        self
    }

    /// Create with field validation errors
    pub fn with_field_errors(mut self, field_errors: Vec<FieldError>) -> Self {
        self.details = Some(serde_json::to_value(field_errors).unwrap_or(Value::Null));
        self
    }

    /// Set the HTTP status code for this error
    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status_code = Some(status);
        self
    }

    /// Set the HTTP status code from an ErrorCode
    pub fn with_status_code(mut self, code: ErrorCode) -> Self {
        self.status_code = Some(
            StatusCode::from_u16(code.status_code()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
        );
        self
    }

    // === Common Error Constructors ===

    /// 400 Bad Request
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::BadRequest, message).with_status(StatusCode::BAD_REQUEST)
    }

    /// 401 Unauthorized
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::Unauthorized, message).with_status(StatusCode::UNAUTHORIZED)
    }

    /// Create an unauthorized error with an auth error code
    pub fn auth(message: impl Into<String>, auth_code: AuthErrorCode) -> Self {
        let code = auth_code.parent_error_code();
        Self::new(code, message)
            .with_details(serde_json::json!({ "auth_code": format!("{:?}", auth_code) }))
            .with_status(StatusCode::UNAUTHORIZED)
    }

    /// 403 Forbidden
    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::Forbidden, message).with_status(StatusCode::FORBIDDEN)
    }

    /// Create a forbidden error with an auth error code (for insufficient permissions)
    pub fn forbidden_with_code(message: impl Into<String>, auth_code: AuthErrorCode) -> Self {
        let code = auth_code.parent_error_code();
        Self::new(code, message)
            .with_details(serde_json::json!({ "auth_code": format!("{:?}", auth_code) }))
            .with_status(StatusCode::FORBIDDEN)
    }

    /// 404 Not Found
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::NotFound, message).with_status(StatusCode::NOT_FOUND)
    }

    /// 409 Conflict
    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::Conflict, message).with_status(StatusCode::CONFLICT)
    }

    /// 422 Unprocessable Entity (validation errors)
    pub fn validation_error(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::ValidationError, message).with_status(StatusCode::UNPROCESSABLE_ENTITY)
    }

    /// Create a validation error with field details
    pub fn validation_error_with_fields(
        message: impl Into<String>,
        field_errors: Vec<FieldError>,
    ) -> Self {
        Self::new(ErrorCode::ValidationError, message)
            .with_details(serde_json::to_value(field_errors).unwrap_or(Value::Null))
            .with_status(StatusCode::UNPROCESSABLE_ENTITY)
    }

    /// 429 Too Many Requests (rate limiting)
    pub fn rate_limited(message: impl Into<String>) -> Self {
        Self::rate_limited_with_retry(message, 60)
    }

    /// 429 Too Many Requests with retry information
    pub fn rate_limited_with_retry(message: impl Into<String>, retry_after_seconds: u64) -> Self {
        Self::new(ErrorCode::RateLimited, message)
            .with_details(serde_json::json!({ "retry_after_seconds": retry_after_seconds }))
            .with_status(StatusCode::TOO_MANY_REQUESTS)
    }

    /// 500 Internal Server Error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::InternalError, message).with_status(StatusCode::INTERNAL_SERVER_ERROR)
    }

    /// 500 Internal Server Error with details
    pub fn internal_with_details(message: impl Into<String>, details: impl Into<String>) -> Self {
        let message = message.into();
        let details = details.into();
        Self::new(
            ErrorCode::InternalError,
            format!("{}: {}", message, details),
        )
        .with_status(StatusCode::INTERNAL_SERVER_ERROR)
    }

    /// 503 Service Unavailable
    pub fn service_unavailable(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::ServiceUnavailable, message)
            .with_status(StatusCode::SERVICE_UNAVAILABLE)
    }

    /// 502 Bad Gateway
    pub fn bad_gateway(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::BadGateway, message).with_status(StatusCode::BAD_GATEWAY)
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code.as_str(), self.message)
    }
}

impl std::error::Error for ApiError {}

impl ErrorCode {
    /// Get the string representation of the error code
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BadRequest => "BAD_REQUEST",
            Self::Unauthorized => "UNAUTHORIZED",
            Self::Forbidden => "FORBIDDEN",
            Self::NotFound => "NOT_FOUND",
            Self::Conflict => "CONFLICT",
            Self::ValidationError => "VALIDATION_ERROR",
            Self::RateLimited => "RATE_LIMITED",
            Self::InternalError => "INTERNAL_ERROR",
            Self::ServiceUnavailable => "SERVICE_UNAVAILABLE",
            Self::BadGateway => "BAD_GATEWAY",
        }
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Result type for API responses
///
/// This is the standard result type for HTTP handlers in the application.
/// All handler functions should return `ApiResult<T>` where `T` is the response data type.
pub type ApiResult<T = Value> = Result<T, ApiError>;

impl IntoResponse for ApiError {
    fn into_response(self) -> Response<Body> {
        let status = self
            .status_code
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        // Create a simple error response using the local ApiError
        #[derive(serde::Serialize)]
        struct ErrorResponse {
            success: bool,
            error: ApiError,
        }

        let response = ErrorResponse {
            success: false,
            error: self,
        };

        (status, Json(response)).into_response()
    }
}

// Convert AppError to ApiError
impl From<crate::core::AppError> for ApiError {
    fn from(error: crate::core::AppError) -> Self {
        match error {
            crate::core::AppError::ValidationError(e) => e.into(),
            crate::core::AppError::AuthenticationError(e) => e.into(),
            crate::core::AppError::AuthorizationError(e) => e.into(),
            crate::core::AppError::NotFoundError(e) => e.into(),
            crate::core::AppError::ConflictError(e) => e.into(),
            crate::core::AppError::RateLimitError(e) => e.into(),
            crate::core::AppError::BusinessError(e) => e.into(),
            crate::core::AppError::ExternalServiceError(e) => e.into(),
            crate::core::AppError::DatabaseError(e) => e.into(),
            crate::core::AppError::InfrastructureError(e) => e.into(),
            crate::core::AppError::InternalError(e) => e.into(),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use axum::http::StatusCode;

//     #[test]
//     fn test_api_error_bad_request() {
//         let err = ApiError::bad_request("Invalid input");
//         assert_eq!(err.code, ErrorCode::BadRequest);
//         assert_eq!(err.status_code, Some(StatusCode::BAD_REQUEST));
//         assert_eq!(err.message, "Invalid input");
//     }

//     #[test]
//     fn test_api_error_not_found() {
//         let err = ApiError::not_found("User not found");
//         assert_eq!(err.code, ErrorCode::NotFound);
//         assert_eq!(err.status_code, Some(StatusCode::NOT_FOUND));
//     }

//     #[test]
//     fn test_api_error_auth() {
//         let err = ApiError::auth("Invalid credentials", AuthErrorCode::InvalidCredentials);
//         assert_eq!(err.code, ErrorCode::Unauthorized);
//         assert_eq!(err.status_code, Some(StatusCode::UNAUTHORIZED));
//         assert!(err.details.is_some());
//     }

//     #[test]
//     fn test_api_error_validation_with_fields() {
//         let field_errors = vec![
//             FieldError {
//                 field: "email".to_string(),
//                 message: "Invalid email".to_string(),
//             },
//             FieldError {
//                 field: "password".to_string(),
//                 message: "Too short".to_string(),
//             },
//         ];
//         let err = ApiError::validation_error_with_fields("Validation failed", field_errors);
//         assert_eq!(err.code, ErrorCode::ValidationError);
//         assert_eq!(err.status_code, Some(StatusCode::UNPROCESSABLE_ENTITY));
//     }

//     #[test]
//     fn test_api_error_rate_limited() {
//         let err = ApiError::rate_limited("Too many requests", 60);
//         assert_eq!(err.code, ErrorCode::RateLimited);
//         assert_eq!(err.status_code, Some(StatusCode::TOO_MANY_REQUESTS));
//         assert!(err.details.is_some());
//     }

//     #[test]
//     fn test_error_code_status_codes() {
//         assert_eq!(ErrorCode::BadRequest.status_code(), 400);
//         assert_eq!(ErrorCode::Unauthorized.status_code(), 401);
//         assert_eq!(ErrorCode::Forbidden.status_code(), 403);
//         assert_eq!(ErrorCode::NotFound.status_code(), 404);
//         assert_eq!(ErrorCode::Conflict.status_code(), 409);
//         assert_eq!(ErrorCode::ValidationError.status_code(), 422);
//         assert_eq!(ErrorCode::RateLimited.status_code(), 429);
//         assert_eq!(ErrorCode::InternalError.status_code(), 500);
//         assert_eq!(ErrorCode::ServiceUnavailable.status_code(), 503);
//         assert_eq!(ErrorCode::BadGateway.status_code(), 502);
//     }

//     #[test]
//     fn test_auth_error_code_parent() {
//         assert_eq!(
//             AuthErrorCode::InsufficientPermissions.parent_error_code(),
//             ErrorCode::Forbidden
//         );
//         assert_eq!(
//             AuthErrorCode::InvalidCredentials.parent_error_code(),
//             ErrorCode::Unauthorized
//         );
//         assert_eq!(
//             AuthErrorCode::TokenExpired.parent_error_code(),
//             ErrorCode::Unauthorized
//         );
//     }
// }
