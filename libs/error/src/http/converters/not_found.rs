//! Converter for not found errors to HTTP API errors

use crate::core::kinds::NotFoundError;
use crate::http::ApiError;

impl From<NotFoundError> for ApiError {
    fn from(error: NotFoundError) -> Self {
        ApiError::not_found(format!("{} not found: {}", error.resource, error.id))
    }
}
