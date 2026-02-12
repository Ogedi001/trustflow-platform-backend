//! Validator adapter error conversions

use crate::core::AppError;

/// Convert from validator::ValidationErrors
impl From<validator::ValidationErrors> for AppError {
    fn from(e: validator::ValidationErrors) -> Self {
        let message = e.to_string();
        Self::validation(message)
    }
}
