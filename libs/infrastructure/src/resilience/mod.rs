//! Resilience patterns for distributed systems
//!
//! Provides production-grade implementations of common resilience patterns:
//! - Circuit Breaker: Prevent cascading failures
//! - Retry: Intelligent retry with exponential backoff
//! - Timeout: Request timeout enforcement
//! - Bulkhead: Resource isolation
//!
//! ## Example
//!
//! ```rust,no_run
//! use infrastructure::resilience::{CircuitBreaker, CircuitBreakerConfig};
//!
//! let cb = CircuitBreaker::new(CircuitBreakerConfig::default());
//! let result = cb.call(|| async {
//!     // Your operation here
//!     Ok(())
//! }).await;
//! ```

pub mod bulkhead;
pub mod circuit_breaker;
pub mod retry;
pub mod timeout;

pub use bulkhead::{Bulkhead, BulkheadConfig};
pub use circuit_breaker::{CircuitBreaker, CircuitBreakerConfig, CircuitBreakerState};
pub use retry::{ExponentialBackoff, RetryConfig, RetryPolicy};
pub use timeout::TimeoutError;
