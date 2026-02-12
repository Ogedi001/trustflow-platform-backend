//! Converter for internal errors to HTTP API errors

use crate::core::kinds::InternalError;
use crate::http::ApiError;

impl From<InternalError> for ApiError {
    fn from(error: InternalError) -> Self {
        ApiError::internal(error.message)
    }
}
