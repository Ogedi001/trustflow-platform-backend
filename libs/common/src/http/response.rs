use super::meta::Meta;
use axum::{Json, http::StatusCode, response::IntoResponse};
use error::http::ApiError;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize, Clone)]

/// Standard API response wrapper for all endpoints
///
/// Always returns JSON with consistent shape for both success and error.
pub struct ApiResponse<T = Value> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,

    #[serde(skip)]
    pub status_code: Option<StatusCode>,
}

impl<T> ApiResponse<T> {
    pub fn success(message: impl Into<String>, data: T) -> Self {
        Self {
            success: true,
            message: Some(message.into()),
            data: Some(data),
            error: None,
            meta: None,
            status_code: Some(StatusCode::OK),
        }
    }

    pub fn success_message(message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: Some(message.into()),
            data: None,
            error: None,
            meta: None,
            status_code: Some(StatusCode::OK),
        }
    }

    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status_code = Some(status);
        self
    }

    pub fn error(error: ApiError) -> Self {
        Self {
            success: false,
            message: None,
            data: None,
            error: Some(error),
            meta: None,
            status_code: None,
        }
    }

    pub fn with_meta(mut self, meta: Meta) -> Self {
        self.meta = Some(meta);
        self
    }
}

pub type ApiResult<T = Value> = Result<ApiResponse<T>, ApiError>;

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code.unwrap_or_else(|| {
            if self.success {
                StatusCode::OK
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        });

        (status, Json(self)).into_response()
    }
}
