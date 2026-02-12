//! Reqwest adapter error conversions

use crate::core::AppError;

/// Convert from reqwest::Error
impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        Self::external("http_client", e.to_string())
    }
}
