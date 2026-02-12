//! SQLx adapter error conversions

use crate::core::AppError;

/// Convert from sqlx::Error
impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        Self::database(e.to_string())
    }
}
