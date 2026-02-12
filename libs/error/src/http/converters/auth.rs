//! Converter for authentication/authorization errors to HTTP API errors

use crate::core::kinds::AuthError;
use crate::http::ApiError;

impl From<AuthError> for ApiError {
    fn from(error: AuthError) -> Self {
        match error {
            AuthError::Authentication { message, code } => ApiError::auth(message, code),
            AuthError::Authorization { message, code } => {
                ApiError::forbidden_with_code(message, code)
            }
        }
    }
}
