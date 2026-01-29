use axum::{
    Json,
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use serde::Serialize;

use super::{ApiError, ApiResponse};

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response<Body> {
        let status = self.status_code.unwrap_or_else(|| {
            if self.success {
                StatusCode::OK
            } else {
                // Use status from error if available, otherwise default to 500
                self.error
                    .as_ref()
                    .and_then(|e| e.status_code)
                    .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            }
        });

        (status, Json(self)).into_response()
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response<Body> {
        let status = self
            .status_code
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let response: ApiResponse<()> = ApiResponse::error(self);
        (status, Json(response)).into_response()
    }
}
