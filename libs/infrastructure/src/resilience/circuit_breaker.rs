//! Circuit Breaker pattern implementation
//!
//! Prevents cascading failures by intercepting calls and tracking their state.
//! Transitions between three states: Closed, Open, and Half-Open.

use std::sync::Arc;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use thiserror::Error;
use tracing::{info, warn};

/// Circuit breaker state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitBreakerState {
    /// Normal state, requests pass through
    Closed,
    /// Failed too many times, requests are blocked
    Open,
    /// Testing if service recovered
    HalfOpen,
}

impl std::fmt::Display for CircuitBreakerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Closed => write!(f, "Closed"),
            Self::Open => write!(f, "Open"),
            Self::HalfOpen => write!(f, "HalfOpen"),
        }
    }
}

/// Circuit breaker error
#[derive(Debug, Error, Clone)]
pub enum CircuitBreakerError {
    #[error("Circuit breaker is open")]
    Open,
    #[error("Failed to execute: {0}")]
    ExecutionError(String),
}

/// Circuit breaker configuration
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    /// Number of failures before opening
    pub failure_threshold: u32,
    /// Number of successes in half-open state before closing
    pub success_threshold: u32,
    /// Duration to wait before transitioning from open to half-open
    pub timeout: Duration,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 2,
            timeout: Duration::from_secs(60),
        }
    }
}

/// Circuit breaker implementation
#[derive(Clone)]
pub struct CircuitBreaker {
    config: Arc<CircuitBreakerConfig>,
    state: Arc<AtomicU32>, // 0=Closed, 1=Open, 2=HalfOpen
    failures: Arc<AtomicU64>,
    successes: Arc<AtomicU64>,
    last_failure_time: Arc<AtomicU64>,
}

impl CircuitBreaker {
    /// Create a new circuit breaker
    pub fn new(config: CircuitBreakerConfig) -> Self {
        Self {
            config: Arc::new(config),
            state: Arc::new(AtomicU32::new(0)), // Closed
            failures: Arc::new(AtomicU64::new(0)),
            successes: Arc::new(AtomicU64::new(0)),
            last_failure_time: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Get current state
    pub fn state(&self) -> CircuitBreakerState {
        match self.state.load(Ordering::Acquire) {
            0 => CircuitBreakerState::Closed,
            1 => CircuitBreakerState::Open,
            2 => CircuitBreakerState::HalfOpen,
            _ => CircuitBreakerState::Closed,
        }
    }

    /// Get failure count
    pub fn failure_count(&self) -> u64 {
        self.failures.load(Ordering::Acquire)
    }

    /// Get success count
    pub fn success_count(&self) -> u64 {
        self.successes.load(Ordering::Acquire)
    }

    /// Execute a function with circuit breaker protection
    pub async fn call<F, Fut, T>(&self, f: F) -> Result<T, CircuitBreakerError>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, Box<dyn std::error::Error + Send + Sync>>>,
    {
        let state = self.state();

        match state {
            CircuitBreakerState::Open => {
                if self.should_attempt_reset() {
                    self.transition_to_half_open();
                    self.execute_call(f).await
                } else {
                    warn!("Circuit breaker is open");
                    Err(CircuitBreakerError::Open)
                }
            }
            CircuitBreakerState::Closed | CircuitBreakerState::HalfOpen => {
                self.execute_call(f).await
            }
        }
    }

    async fn execute_call<F, Fut, T>(&self, f: F) -> Result<T, CircuitBreakerError>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, Box<dyn std::error::Error + Send + Sync>>>,
    {
        match f().await {
            Ok(result) => {
                self.record_success();
                Ok(result)
            }
            Err(e) => {
                self.record_failure();
                Err(CircuitBreakerError::ExecutionError(e.to_string()))
            }
        }
    }

    fn record_success(&self) {
        let state = self.state();
        match state {
            CircuitBreakerState::Closed => {
                // Reset failures on success in closed state
                self.failures.store(0, Ordering::Release);
            }
            CircuitBreakerState::HalfOpen => {
                let successes = self.successes.fetch_add(1, Ordering::AcqRel) + 1;
                if successes >= self.config.success_threshold as u64 {
                    self.transition_to_closed();
                }
            }
            _ => {}
        }
    }

    fn record_failure(&self) {
        let state = self.state();
        match state {
            CircuitBreakerState::Closed => {
                let failures = self.failures.fetch_add(1, Ordering::AcqRel) + 1;
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                self.last_failure_time.store(now, Ordering::Release);

                if failures >= self.config.failure_threshold as u64 {
                    self.transition_to_open();
                }
            }
            CircuitBreakerState::HalfOpen => {
                self.transition_to_open();
            }
            _ => {}
        }
    }

    fn should_attempt_reset(&self) -> bool {
        let last_failure = self.last_failure_time.load(Ordering::Acquire);
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let elapsed = Duration::from_secs(now - last_failure);
        elapsed >= self.config.timeout
    }

    fn transition_to_closed(&self) {
        self.state.store(0, Ordering::Release);
        self.failures.store(0, Ordering::Release);
        self.successes.store(0, Ordering::Release);
        info!("Circuit breaker transitioned to Closed");
    }

    fn transition_to_open(&self) {
        self.state.store(1, Ordering::Release);
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.last_failure_time.store(now, Ordering::Release);
        warn!("Circuit breaker transitioned to Open");
    }

    fn transition_to_half_open(&self) {
        self.state.store(2, Ordering::Release);
        self.successes.store(0, Ordering::Release);
        info!("Circuit breaker transitioned to HalfOpen");
    }

    /// Reset the circuit breaker to closed state
    pub fn reset(&self) {
        self.transition_to_closed();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_circuit_breaker_closes_on_success() {
        let cb = CircuitBreaker::new(CircuitBreakerConfig::default());

        let result = cb.call(|| async { Ok(()) }).await;

        assert!(result.is_ok());
        assert_eq!(cb.state(), CircuitBreakerState::Closed);
    }

    #[tokio::test]
    async fn test_circuit_breaker_opens_on_threshold() {
        let config = CircuitBreakerConfig {
            failure_threshold: 2,
            ..Default::default()
        };
        let cb = CircuitBreaker::new(config);

        // Trigger failures
        for _ in 0..2 {
            let _ = cb
                .call(|| async {
                    Err(
                        Box::new(std::io::Error::new(std::io::ErrorKind::Other, "test"))
                            as Box<dyn std::error::Error + Send + Sync>,
                    )
                })
                .await;
        }

        assert_eq!(cb.state(), CircuitBreakerState::Open);
    }

    #[test]
    fn test_circuit_breaker_blocks_when_open() {
        let cb = CircuitBreaker::new(CircuitBreakerConfig::default());
        cb.transition_to_open();

        // Block until timeout
        assert_eq!(cb.state(), CircuitBreakerState::Open);
    }
}
