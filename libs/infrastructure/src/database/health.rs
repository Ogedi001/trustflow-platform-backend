//! Database health check utilities
//!
//! Provides health check implementations for database connectivity and performance monitoring.

#[cfg(feature = "database")]
use async_trait::async_trait;
#[cfg(feature = "database")]
use std::time::Instant;
#[cfg(feature = "database")]
use thiserror::Error;
#[cfg(feature = "database")]
use tracing::{info, warn};

#[cfg(feature = "database")]
use super::DbPool;

/// Health check error
#[cfg(feature = "database")]
#[derive(Debug, Error, Clone)]
pub enum HealthCheckError {
    #[error("Database connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Health check query failed: {0}")]
    QueryFailed(String),

    #[error("Health check timed out")]
    Timeout,
}

/// Health check result
#[cfg(feature = "database")]
#[derive(Debug, Clone)]
pub struct HealthCheckResult {
    /// Whether the check passed
    pub healthy: bool,
    /// Time taken in milliseconds
    pub latency_ms: u128,
    /// Number of available connections
    pub available_connections: Option<u32>,
    /// Error message if check failed
    pub error: Option<String>,
}

impl std::fmt::Display for HealthCheckResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "healthy: {}, latency: {}ms",
            self.healthy, self.latency_ms
        )
    }
}

/// Database health checker
#[cfg(feature = "database")]
pub struct HealthChecker {
    pool: DbPool,
}

#[cfg(feature = "database")]
impl HealthChecker {
    /// Create a new health checker
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Check database connectivity
    pub async fn check(&self) -> HealthCheckResult {
        let start = Instant::now();

        match self.execute_health_check().await {
            Ok(_) => {
                let latency_ms = start.elapsed().as_millis();
                info!("Database health check passed in {}ms", latency_ms);

                HealthCheckResult {
                    healthy: true,
                    latency_ms,
                    available_connections: self.get_available_connections(),
                    error: None,
                }
            }
            Err(e) => {
                let latency_ms = start.elapsed().as_millis();
                let error_msg = e.to_string();
                warn!(
                    "Database health check failed in {}ms: {}",
                    latency_ms, error_msg
                );

                HealthCheckResult {
                    healthy: false,
                    latency_ms,
                    available_connections: None,
                    error: Some(error_msg),
                }
            }
        }
    }

    /// Perform detailed health check with metrics
    pub async fn detailed_check(&self) -> DetailedHealthCheckResult {
        let start = Instant::now();
        let simple_check = self.check().await;

        DetailedHealthCheckResult {
            simple: simple_check,
            uptime_check_ms: start.elapsed().as_millis(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    async fn execute_health_check(&self) -> Result<(), HealthCheckError> {
        sqlx::query("SELECT 1")
            .fetch_one(self.pool.pool())
            .await
            .map_err(|e| HealthCheckError::QueryFailed(e.to_string()))?;

        Ok(())
    }

    fn get_available_connections(&self) -> Option<u32> {
        // SQLx doesn't expose connection count directly, but we can check pool size
        Some(self.pool.pool().num_idle() as u32)
    }
}

/// Detailed health check result with timestamp
#[cfg(feature = "database")]
#[derive(Debug, Clone)]
pub struct DetailedHealthCheckResult {
    /// Simple health check result
    pub simple: HealthCheckResult,
    /// Total uptime check time in milliseconds
    pub uptime_check_ms: u128,
    /// Timestamp of the check (RFC3339)
    pub timestamp: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_check_result_display() {
        let result = HealthCheckResult {
            healthy: true,
            latency_ms: 5,
            available_connections: Some(10),
            error: None,
        };

        let display = result.to_string();
        assert!(display.contains("healthy: true"));
        assert!(display.contains("latency: 5ms"));
    }
}
