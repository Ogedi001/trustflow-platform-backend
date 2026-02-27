//! Bulkhead pattern implementation for resource isolation
//!
//! Limits concurrent executions to prevent resource exhaustion.

use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use thiserror::Error;
use tokio::sync::Semaphore;
use tracing::warn;

/// Bulkhead error
#[derive(Debug, Error, Clone)]
pub enum BulkheadError {
    #[error("Bulkhead rejected: maximum {max} concurrent requests, {current} in use")]
    Rejected { max: u32, current: u32 },
}

/// Bulkhead configuration
#[derive(Debug, Clone)]
pub struct BulkheadConfig {
    /// Maximum number of concurrent requests
    pub max_concurrent: usize,
}

impl Default for BulkheadConfig {
    fn default() -> Self {
        Self {
            max_concurrent: 10,
        }
    }
}

/// Bulkhead implementation using semaphore
pub struct Bulkhead {
    semaphore: Arc<Semaphore>,
    max_concurrent: u32,
    current_count: Arc<AtomicU32>,
}

impl Clone for Bulkhead {
    fn clone(&self) -> Self {
        Self {
            semaphore: self.semaphore.clone(),
            max_concurrent: self.max_concurrent,
            current_count: self.current_count.clone(),
        }
    }
}

impl Bulkhead {
    /// Create a new bulkhead
    pub fn new(config: BulkheadConfig) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(config.max_concurrent)),
            max_concurrent: config.max_concurrent as u32,
            current_count: Arc::new(AtomicU32::new(0)),
        }
    }

    /// Get current concurrent count
    pub fn current_count(&self) -> u32 {
        self.current_count.load(Ordering::Acquire)
    }

    /// Execute a function with bulkhead protection
    pub async fn call<F, Fut, T>(&self, f: F) -> Result<T, BulkheadError>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        // Try to acquire permit
        let permit = self.semaphore.acquire().await;
        match permit {
            Ok(_permit) => {
                let current = self.current_count.fetch_add(1, Ordering::AcqRel) + 1;
                
                let result = f().await;
                
                self.current_count.fetch_sub(1, Ordering::AcqRel);
                Ok(result)
            }
            Err(_) => {
                let current = self.current_count.load(Ordering::Acquire);
                warn!(
                    "Bulkhead rejected: maximum {} concurrent requests, {} in use",
                    self.max_concurrent, current
                );
                Err(BulkheadError::Rejected {
                    max: self.max_concurrent,
                    current,
                })
            }
        }
    }

    /// Try to execute without waiting
    pub fn try_call<F, T>(&self, f: F) -> Result<T, BulkheadError>
    where
        F: FnOnce() -> T,
    {
        match self.semaphore.try_acquire() {
            Ok(_permit) => {
                self.current_count.fetch_add(1, Ordering::AcqRel);
                let result = f();
                self.current_count.fetch_sub(1, Ordering::AcqRel);
                Ok(result)
            }
            Err(_) => {
                let current = self.current_count.load(Ordering::Acquire);
                warn!(
                    "Bulkhead rejected: maximum {} concurrent requests, {} in use",
                    self.max_concurrent, current
                );
                Err(BulkheadError::Rejected {
                    max: self.max_concurrent,
                    current,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;
    use std::time::Duration;

    #[tokio::test]
    async fn test_bulkhead_allows_concurrent() {
        let bulkhead = Bulkhead::new(BulkheadConfig {
            max_concurrent: 2,
        });

        let result1 = bulkhead.call(|| async { 1 }).await;
        let result2 = bulkhead.call(|| async { 2 }).await;

        assert!(result1.is_ok());
        assert!(result2.is_ok());
    }

    #[tokio::test]
    async fn test_bulkhead_rejects_beyond_limit() {
        let bulkhead = Bulkhead::new(BulkheadConfig {
            max_concurrent: 1,
        });

        let _guard = bulkhead.semaphore.acquire().await.unwrap();
        let result = bulkhead.call(|| async { 1 }).await;

        assert!(result.is_err());
    }

    #[test]
    fn test_bulkhead_try_call_rejects() {
        let bulkhead = Bulkhead::new(BulkheadConfig {
            max_concurrent: 1,
        });

        let _guard = bulkhead.semaphore.try_acquire();
        let result = bulkhead.try_call(|| 1);

        assert!(result.is_err());
    }
}
