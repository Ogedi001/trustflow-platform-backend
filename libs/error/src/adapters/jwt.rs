//! JWT adapter error conversions

use crate::core::AppError;

/// Convert from jsonwebtoken::errors::Error
impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        Self::auth(
            e.to_string(),
            crate::http::error_code::AuthErrorCode::TokenInvalid,
        )
    }
}
