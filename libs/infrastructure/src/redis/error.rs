//! Redis error types
//!
//! Provides comprehensive error types for Redis operations with proper
//! error categorization, conversion from underlying Redis errors, and
//! integration with the application's error handling system.
//!
//! ## Error Categories
//!
//! - `Connection`: Failed to connect or lost connection to Redis
//! - `Command`: Error executing a Redis command
//! - `Serialization`: Error serializing data to Redis
//! - `Deserialization`: Error deserializing data from Redis
//! - `Pool`: Error with connection pool management
//! - `Timeout`: Operation timed out
//! - `NotFound`: Key not found in Redis

/// Comprehensive Redis error type
///
/// This enum is intentionally expressive so that callers can react differently
/// to connection problems, serialization failures, timeouts, etc.  When the
/// infrastructure crate is used from services the errors are converted into
/// the shared `AppError` type by implementing `From<RedisError>`.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum RedisError {
    /// Failed to connect to Redis or connection lost
    #[error("connection error: {message}")]
    Connection {
        /// Detailed error message
        message: String,
    },

    /// Error executing a Redis command
    #[error("command {command} failed: {message}")]
    Command {
        /// The command that failed
        command: String,
        /// Detailed error message
        message: String,
    },

    /// Error serializing data for storage in Redis
    #[error("serialization error for {target_type}: {message}")]
    Serialization {
        /// The type being serialized
        target_type: String,
        /// Detailed error message
        message: String,
    },

    /// Error deserializing data from Redis
    #[error("deserialization error for {target_type}: {message}")]
    Deserialization {
        /// The type being deserialized
        target_type: String,
        /// Detailed error message
        message: String,
    },

    /// Error with connection pool or manager
    #[error("pool error during {operation}: {message}")]
    Pool {
        /// Operation that caused the pool error
        operation: String,
        /// Detailed error message
        message: String,
    },

    /// Operation timed out
    #[error("timeout after {duration_ms}ms during {operation}")]
    Timeout {
        /// Operation that timed out
        operation: String,
        /// Timeout duration
        duration_ms: u64,
    },

    /// Key not found in Redis
    #[error("key not found: {key}")]
    NotFound {
        /// The key that was not found
        key: String,
    },

    /// Authentication failure
    #[error("authentication failure: {message}")]
    Authentication {
        /// Detailed error message
        message: String,
    },

    /// Invalid configuration
    #[error("configuration error for '{parameter}': {message}")]
    Configuration {
        /// Configuration parameter that is invalid
        parameter: String,
        /// Detailed error message
        message: String,
    },

    /// Catch-all for other Redis errors
    #[error("{0}")]
    Other(String),
}

impl RedisError {
    // ===== Constructor methods =====

    /// Create a new connection error
    pub fn connection(message: impl Into<String>) -> Self {
        Self::Connection {
            message: message.into(),
        }
    }

    /// Create a new command error
    pub fn command(command: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Command {
            command: command.into(),
            message: message.into(),
        }
    }

    /// Create a new serialization error
    pub fn serialization(target_type: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Serialization {
            target_type: target_type.into(),
            message: message.into(),
        }
    }

    /// Create a new deserialization error
    pub fn deserialization(target_type: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Deserialization {
            target_type: target_type.into(),
            message: message.into(),
        }
    }

    /// Create a new pool error
    pub fn pool(operation: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Pool {
            operation: operation.into(),
            message: message.into(),
        }
    }

    /// Create a new timeout error
    pub fn timeout(operation: impl Into<String>, duration_ms: u64) -> Self {
        Self::Timeout {
            operation: operation.into(),
            duration_ms,
        }
    }

    /// Create a new not found error
    pub fn not_found(key: impl Into<String>) -> Self {
        Self::NotFound { key: key.into() }
    }

    /// Create a new authentication error
    pub fn authentication(message: impl Into<String>) -> Self {
        Self::Authentication {
            message: message.into(),
        }
    }

    /// Create a new configuration error
    pub fn configuration(parameter: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Configuration {
            parameter: parameter.into(),
            message: message.into(),
        }
    }

    /// Create a generic other error
    pub fn other(message: impl Into<String>) -> Self {
        Self::Other(message.into())
    }

    // ===== Helper methods =====

    /// Check if this is a connection error
    pub fn is_connection(&self) -> bool {
        matches!(self, Self::Connection { .. })
    }

    /// Check if this is a not found error
    pub fn is_not_found(&self) -> bool {
        matches!(self, Self::NotFound { .. })
    }

    /// Check if this is a timeout error
    pub fn is_timeout(&self) -> bool {
        matches!(self, Self::Timeout { .. })
    }

    /// Convert from a redis::RedisError using pattern matching on the message
    /// This avoids the conflicting From implementation in the redis crate
    pub fn from_redis_error(e: &::redis::RedisError) -> Self {
        let error_str = e.to_string().to_lowercase();

        if error_str.contains("connection") || error_str.contains("connect") {
            Self::connection(e.to_string())
        } else if error_str.contains("authentication") || error_str.contains("auth") {
            Self::authentication(e.to_string())
        } else if error_str.contains("timeout") || error_str.contains("timed out") {
            Self::timeout("Redis operation", 0)
        } else if error_str.contains("not found") {
            Self::command("Unknown", e.to_string())
        } else {
            Self::command("Unknown", e.to_string())
        }
    }
}

// ===== From implementations for common error types =====

impl From<std::io::Error> for RedisError {
    fn from(e: std::io::Error) -> Self {
        Self::connection(e.to_string())
    }
}

impl From<serde_json::Error> for RedisError {
    fn from(e: serde_json::Error) -> Self {
        Self::serialization("JSON", e.to_string())
    }
}

impl From<bb8::RunError<::redis::RedisError>> for RedisError {
    fn from(e: bb8::RunError<::redis::RedisError>) -> Self {
        match e {
            bb8::RunError::User(err) => RedisError::from_redis_error(&err),
            bb8::RunError::TimedOut => RedisError::timeout("bb8 pool", 0),
        }
    }
}

impl From<::redis::RedisError> for RedisError {
    fn from(e: ::redis::RedisError) -> Self {
        RedisError::from_redis_error(&e)
    }
}

// ===== Integration with application-wide error type =====

impl From<RedisError> for crate::AppError {
    fn from(e: RedisError) -> crate::AppError {
        crate::AppError::infrastructure("redis", e.to_string())
    }
}

// ===== Result type alias =====

/// Result type for Redis operations
pub type RedisResult<T = ()> = Result<T, RedisError>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AppError;
    use ::redis::RedisError as ExternalRedisError;

    #[test]
    fn from_io_error_becomes_connection() {
        let io = std::io::Error::new(std::io::ErrorKind::Other, "oops");
        let err: RedisError = io.into();
        assert!(matches!(err, RedisError::Connection { .. }));
    }

    #[test]
    fn redis_crate_error_mapping() {
        // simulate a connection error message
        let underlying =
            ExternalRedisError::from((::redis::ErrorKind::IoError, "connection refused"));
        let err: RedisError = underlying.into();
        assert!(err.is_connection());

        let auth = ExternalRedisError::from((
            ::redis::ErrorKind::AuthenticationFailed,
            "invalid password",
        ));
        let err2: RedisError = auth.into();
        assert!(matches!(err2, RedisError::Authentication { .. }));
    }

    #[test]
    fn to_app_error_wraps() {
        let re = RedisError::timeout("cmd", 123);
        let app: AppError = re.clone().into();
        if let AppError::InfrastructureError(infra) = app {
            assert!(infra.message.contains("timeout"));
        } else {
            panic!("expected infrastructure error");
        }
    }
}
