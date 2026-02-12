//! Redis adapter error conversions

use crate::core::AppError;

/// Convert from redis::RedisError
impl From<redis::RedisError> for AppError {
    fn from(e: redis::RedisError) -> Self {
        Self::infrastructure("redis", e.to_string())
    }
}
