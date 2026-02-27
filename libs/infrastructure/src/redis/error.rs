//! Redis error types.

use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum RedisError {
    #[error("redis connection error: {0}")]
    Connection(String),

    #[error("redis command error: {0}")]
    Command(String),

    #[error("redis configuration error: {0}")]
    Configuration(String),
}

impl From<redis::RedisError> for RedisError {
    fn from(value: redis::RedisError) -> Self {
        Self::Command(value.to_string())
    }
}
