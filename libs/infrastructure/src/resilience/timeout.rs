//! Timeout implementation for async operations
//!
//! Provides enforced timeouts for operations that may hang or take too long.

use std::future::Future;
use std::time::Duration;
use thiserror::Error;
use tracing::warn;

/// Timeout error
#[derive(Debug, Error, Clone)]
pub enum TimeoutError {
    #[error("Operation timed out after {duration_ms}ms")]
    Exceeded { duration_ms: u64 },
}

/// Execute an operation with a timeout
pub async fn with_timeout<F, Fut, T>(
    duration: Duration,
    operation: F,
) -> Result<T, TimeoutError>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = T>,
{
    match tokio::time::timeout(duration, operation()).await {
        Ok(result) => Ok(result),
        Err(_) => {
            warn!(
                "Operation timed out after {}ms",
                duration.as_millis()
            );
            Err(TimeoutError::Exceeded {
                duration_ms: duration.as_millis() as u64,
            })
        }
    }
}

/// Execute an operation with a timeout, returning a default value on timeout
pub async fn with_timeout_or_default<F, Fut, T>(
    duration: Duration,
    operation: F,
    default: T,
) -> T
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = T>,
{
    match tokio::time::timeout(duration, operation()).await {
        Ok(result) => result,
        Err(_) => {
            warn!(
                "Operation timed out after {}ms, returning default",
                duration.as_millis()
            );
            default
        }
    }
}

/// Execute an operation with a timeout, converting timeout to Result
pub async fn with_timeout_result<F, Fut, T, E>(
    duration: Duration,
    operation: F,
) -> Result<T, TimeoutError>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    match tokio::time::timeout(duration, operation()).await {
        Ok(Ok(result)) => Ok(result),
        Ok(Err(_)) => Err(TimeoutError::Exceeded {
            duration_ms: duration.as_millis() as u64,
        }),
        Err(_) => {
            warn!(
                "Operation timed out after {}ms",
                duration.as_millis()
            );
            Err(TimeoutError::Exceeded {
                duration_ms: duration.as_millis() as u64,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_timeout_succeeds() {
        let result = with_timeout(Duration::from_secs(1), || async { 42 }).await;
        assert_eq!(result, Ok(42));
    }

    #[tokio::test]
    async fn test_timeout_exceeds() {
        let result: Result<i32, TimeoutError> = with_timeout(Duration::from_millis(100), || async {
            tokio::time::sleep(Duration::from_secs(1)).await;
            42
        })
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_timeout_or_default() {
        let result = with_timeout_or_default(Duration::from_millis(100), || async {
            tokio::time::sleep(Duration::from_secs(1)).await;
            42
        }, 0).await;
        assert_eq!(result, 0);
    }
}
