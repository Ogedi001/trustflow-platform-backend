//! Metrics collection middleware
//!
//! Collects performance metrics including response times, status codes,
//! and request counts for monitoring and observability.

use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

/// Metrics for HTTP endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointMetrics {
    /// Total requests
    pub total_requests: u64,
    /// Successful requests (2xx)
    pub success_count: u64,
    /// Client errors (4xx)
    pub client_error_count: u64,
    /// Server errors (5xx)
    pub server_error_count: u64,
    /// Total response time in milliseconds
    pub total_response_time_ms: u64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Minimum response time in milliseconds
    pub min_response_time_ms: u64,
    /// Maximum response time in milliseconds
    pub max_response_time_ms: u64,
}

impl EndpointMetrics {
    /// Create new metrics
    pub fn new() -> Self {
        Self {
            total_requests: 0,
            success_count: 0,
            client_error_count: 0,
            server_error_count: 0,
            total_response_time_ms: 0,
            avg_response_time_ms: 0.0,
            min_response_time_ms: u64::MAX,
            max_response_time_ms: 0,
        }
    }

    /// Update with response information
    pub fn update(&mut self, status_code: u16, response_time_ms: u64) {
        self.total_requests += 1;

        match status_code {
            200..=299 => self.success_count += 1,
            400..=499 => self.client_error_count += 1,
            500..=599 => self.server_error_count += 1,
            _ => {}
        }

        self.total_response_time_ms += response_time_ms;
        self.avg_response_time_ms = self.total_response_time_ms as f64 / self.total_requests as f64;
        self.min_response_time_ms = self.min_response_time_ms.min(response_time_ms);
        self.max_response_time_ms = self.max_response_time_ms.max(response_time_ms);
    }
}

impl Default for EndpointMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Atomic metrics counter for concurrent access
#[derive(Debug, Clone)]
pub struct MetricsCollector {
    total_requests: Arc<AtomicU64>,
    success_count: Arc<AtomicU64>,
    error_count: Arc<AtomicU64>,
    total_response_time: Arc<AtomicU64>,
}

impl MetricsCollector {
    /// Create new metrics collector
    pub fn new() -> Self {
        Self {
            total_requests: Arc::new(AtomicU64::new(0)),
            success_count: Arc::new(AtomicU64::new(0)),
            error_count: Arc::new(AtomicU64::new(0)),
            total_response_time: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Record request
    pub fn record_request(&self, status_code: u16, response_time_ms: u64) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        self.total_response_time
            .fetch_add(response_time_ms, Ordering::Relaxed);

        if status_code >= 400 {
            self.error_count.fetch_add(1, Ordering::Relaxed);
        } else {
            self.success_count.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Get total requests
    pub fn total_requests(&self) -> u64 {
        self.total_requests.load(Ordering::Relaxed)
    }

    /// Get success count
    pub fn success_count(&self) -> u64 {
        self.success_count.load(Ordering::Relaxed)
    }

    /// Get error count
    pub fn error_count(&self) -> u64 {
        self.error_count.load(Ordering::Relaxed)
    }

    /// Get average response time
    pub fn avg_response_time_ms(&self) -> f64 {
        let total = self.total_requests();
        if total == 0 {
            return 0.0;
        }
        self.total_response_time.load(Ordering::Relaxed) as f64 / total as f64
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Middleware for metrics collection
pub async fn metrics_middleware(
    req: Request,
    next: Next,
    collector: MetricsCollector,
) -> Result<Response, StatusCode> {
    let start = std::time::Instant::now();
    let response = next.run(req).await;
    let elapsed = start.elapsed();

    collector.record_request(response.status().as_u16(), elapsed.as_millis() as u64);

    Ok(response)
}

/// Create metrics middleware with collector
pub fn make_metrics_middleware(
    collector: MetricsCollector,
) -> impl Fn(Request, Next) -> futures::future::BoxFuture<'static, Result<Response, StatusCode>> + Clone
{
    move |req: Request, next: Next| {
        let collector = collector.clone();
        Box::pin(metrics_middleware(req, next, collector))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_metrics_creation() {
        let metrics = EndpointMetrics::new();
        assert_eq!(metrics.total_requests, 0);
        assert_eq!(metrics.success_count, 0);
    }

    #[test]
    fn test_endpoint_metrics_update() {
        let mut metrics = EndpointMetrics::new();
        metrics.update(200, 100);
        assert_eq!(metrics.total_requests, 1);
        assert_eq!(metrics.success_count, 1);
    }

    #[test]
    fn test_metrics_collector_record() {
        let collector = MetricsCollector::new();
        collector.record_request(200, 50);
        collector.record_request(200, 75);
        collector.record_request(500, 100);

        assert_eq!(collector.total_requests(), 3);
        assert_eq!(collector.success_count(), 2);
        assert_eq!(collector.error_count(), 1);
    }

    #[test]
    fn test_metrics_collector_avg_response_time() {
        let collector = MetricsCollector::new();
        collector.record_request(200, 100);
        collector.record_request(200, 200);

        let avg = collector.avg_response_time_ms();
        assert!((avg - 150.0).abs() < 0.1);
    }
}
