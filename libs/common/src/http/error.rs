use crate::http::ErrorCode;
use axum::http::StatusCode;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ApiError {
    /// Examples: "VALIDATION_ERROR", "NOT_FOUND", "UNAUTHORIZED", "RATE_LIMITED"
    pub code: ErrorCode,
    /// Human-readable error message (safe to display to users)
    pub message: String,

    /// Additional error details (field-specific errors, validation details, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,

    /// HTTP status code (not serialized to JSON, used for HTTP response)
    #[serde(skip)]
    pub status_code: Option<StatusCode>,
}

impl ApiError {
    /// Create a new API error
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            details: None,
            status_code: None,
        }
    }

    /// Create with details
    pub fn with_details<T: Serialize>(mut self, details: T) -> Self {
        self.details = Some(serde_json::to_value(details).unwrap_or(Value::Null));
        self
    }

    /// Create with field validation details
    pub fn with_validation_details(mut self, field_errors: Vec<FieldError>) -> Self {
        self.details = Some(serde_json::to_value(field_errors).unwrap_or(Value::Null));
        self
    }

    /// Set HTTP status code
    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status_code = Some(status);
        self
    }

    // Common error constructors (following HTTP status mapping)

    /// 400 Bad Request
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::BadRequest, message).with_status(StatusCode::BAD_REQUEST)
    }

    /// 401 Unauthorized
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::Unauthorized, message).with_status(StatusCode::UNAUTHORIZED)
    }

    /// 403 Forbidden
    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::Forbidden, message).with_status(StatusCode::FORBIDDEN)
    }

    /// 404 Not Found
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::NotFound, message).with_status(StatusCode::NOT_FOUND)
    }

    /// 409 Conflict
    pub fn conflict(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::Conflict, message).with_status(StatusCode::CONFLICT)
    }

    /// 422 Unprocessable Entity (Stripe-style validation errors)
    pub fn validation_error(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::ValidationError, message).with_status(StatusCode::UNPROCESSABLE_ENTITY)
    }

    /// 429 Too Many Requests
    pub fn rate_limited(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::RateLimited, message).with_status(StatusCode::TOO_MANY_REQUESTS)
    }

    /// 500 Internal Server Error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::InternalError, message).with_status(StatusCode::INTERNAL_SERVER_ERROR)
    }

    /// 503 Service Unavailable
    pub fn service_unavailable(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::ServiceUnavailable, message)
            .with_status(StatusCode::SERVICE_UNAVAILABLE)
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}:{}", self.code, self.message)
    }
}

impl std::error::Error for ApiError {}
