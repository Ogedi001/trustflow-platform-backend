//! Retry policy implementation with exponential backoff
//!
//! Provides configurable retry strategies for transient failures.

use std::future::Future;
use std::time::Duration;
use thiserror::Error;
use tracing::{warn, debug};

/// Retry configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retries
    pub max_retries: u32,
    /// Initial backoff duration
    pub initial_backoff: Duration,
    /// Maximum backoff duration
    pub max_backoff: Duration,
    /// Backoff multiplier
    pub multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_backoff: Duration::from_millis(100),
            max_backoff: Duration::from_secs(30),
            multiplier: 2.0,
        }
    }
}

/// Retry policy
pub struct RetryPolicy {
    config: RetryConfig,
}

impl RetryPolicy {
    /// Create a new retry policy
    pub fn new(config: RetryConfig) -> Self {
        Self { config }
    }

    /// Execute with retry
    pub async fn execute<F, Fut, T, E>(&self, mut f: F) -> Result<T, E>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<T, E>>,
        E: std::fmt::Display,
    {
        let mut backoff = self.config.initial_backoff;
        let mut attempt = 0;

        loop {
            match f().await {
                Ok(result) => {
                    if attempt > 0 {
                        debug!("Operation succeeded after {} retries", attempt);
                    }
                    return Ok(result);
                }
                Err(e) => {
                    attempt += 1;
                    if attempt > self.config.max_retries {
                        warn!("Operation failed after {} attempts: {}", attempt, e);
                        return Err(e);
                    }

                    warn!(
                        "Operation failed (attempt {}/{}), retrying in {:?}: {}",
                        attempt, self.config.max_retries, backoff, e
                    );

                    tokio::time::sleep(backoff).await;
                    backoff = self.calculate_backoff(backoff);
                }
            }
        }
    }

    fn calculate_backoff(&self, current: Duration) -> Duration {
        let next = Duration::from_secs_f64(current.as_secs_f64() * self.config.multiplier);
        next.min(self.config.max_backoff)
    }
}

/// Exponential backoff calculation
pub struct ExponentialBackoff {
    config: RetryConfig,
}

impl ExponentialBackoff {
    /// Create a new exponential backoff calculator
    pub fn new(config: RetryConfig) -> Self {
        Self { config }
    }

    /// Get backoff duration for attempt number
    pub fn duration_for_attempt(&self, attempt: u32) -> Duration {
        let duration_secs = self.config.initial_backoff.as_secs_f64()
            * self.config.multiplier.powi(attempt as i32 - 1);
        Duration::from_secs_f64(duration_secs).min(self.config.max_backoff)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_retry_on_success() {
        let retry = RetryPolicy::new(RetryConfig::default());
        let result: Result<i32, &str> = retry.execute(|| async { Ok(42) }).await;
        assert_eq!(result, Ok(42));
    }

    #[tokio::test]
    async fn test_retry_on_failure() {
        let config = RetryConfig {
            max_retries: 2,
            initial_backoff: Duration::from_millis(10),
            ..Default::default()
        };
        let retry = RetryPolicy::new(config);
        let attempts = Arc::new(AtomicU32::new(0));
        let attempts_clone = attempts.clone();

        let result: Result<i32, &str> = retry
            .execute(|| {
                let attempts = attempts_clone.clone();
                async move {
                    attempts.fetch_add(1, Ordering::SeqCst);
                    if attempts.load(Ordering::SeqCst) < 2 {
                        Err("transient error")
                    } else {
                        Ok(42)
                    }
                }
            })
            .await;

        assert_eq!(result, Ok(42));
        assert_eq!(attempts.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_exponential_backoff() {
        let eb = ExponentialBackoff::new(RetryConfig {
            initial_backoff: Duration::from_secs(1),
            max_backoff: Duration::from_secs(60),
            multiplier: 2.0,
            ..Default::default()
        });

        assert_eq!(eb.duration_for_attempt(1), Duration::from_secs(1));
        assert_eq!(eb.duration_for_attempt(2), Duration::from_secs(2));
        assert_eq!(eb.duration_for_attempt(3), Duration::from_secs(4));
    }
}
