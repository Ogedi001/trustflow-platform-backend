//! JWT adapter error conversions
use crate::core::AppError;

impl From<argon2::password_hash::Error> for AppError {
    fn from(e: argon2::password_hash::Error) -> Self {
        Self::business(e.to_string(), "PASSWORD_HASH_ERROR".to_string())
    }
}
