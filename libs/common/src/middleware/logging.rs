//! Request/response logging middleware
//!
//! Logs HTTP requests and responses with structured tracing for observability.

use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Request logging configuration
#[derive(Debug, Clone)]
pub struct LoggingConfig {
    /// Log request headers
    pub log_headers: bool,
    /// Log request body (may include sensitive data)
    pub log_body: bool,
    /// Log response body
    pub log_response_body: bool,
    /// Paths to exclude from logging
    pub exclude_paths: Vec<String>,
}

impl LoggingConfig {
    /// Create new logging config
    pub fn new() -> Self {
        Self {
            log_headers: true,
            log_body: false,
            log_response_body: false,
            exclude_paths: vec!["/health".to_string(), "/metrics".to_string()],
        }
    }

    /// Enable body logging
    pub fn with_body(mut self) -> Self {
        self.log_body = true;
        self
    }

    /// Enable response body logging
    pub fn with_response_body(mut self) -> Self {
        self.log_response_body = true;
        self
    }

    /// Add path to exclude
    pub fn exclude_path(mut self, path: impl Into<String>) -> Self {
        self.exclude_paths.push(path.into());
        self
    }

    /// Check if path should be logged
    pub fn should_log(&self, path: &str) -> bool {
        !self
            .exclude_paths
            .iter()
            .any(|excluded| path.starts_with(excluded))
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Logged request information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestLog {
    /// HTTP method
    pub method: String,
    /// Request path
    pub path: String,
    /// Request ID (from header if available)
    pub request_id: Option<String>,
    /// Client IP
    pub client_ip: Option<String>,
    /// User agent
    pub user_agent: Option<String>,
}

/// Logged response information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseLog {
    /// HTTP status code
    pub status_code: u16,
    /// Response duration in milliseconds
    pub duration_ms: u64,
    /// Response size in bytes
    pub size_bytes: usize,
}

/// Middleware for request/response logging
pub async fn logging_middleware(
    req: Request,
    next: Next,
    config: LoggingConfig,
) -> Result<Response, StatusCode> {
    let path = req.uri().path().to_string();

    // Check if path should be logged
    if !config.should_log(&path) {
        return Ok(next.run(req).await);
    }

    let method = req.method().to_string();
    let start = Instant::now();

    // Build request log
    let request_log = RequestLog {
        method: method.clone(),
        path: path.clone(),
        request_id: req
            .headers()
            .get("x-request-id")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string()),
        client_ip: req
            .headers()
            .get("x-forwarded-for")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string()),
        user_agent: req
            .headers()
            .get("user-agent")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string()),
    };

    // Log request
    #[cfg(feature = "logging")]
    tracing::info!(
        method = %request_log.method,
        path = %request_log.path,
        request_id = ?request_log.request_id,
        "HTTP request started"
    );

    let response = next.run(req).await;
    let elapsed = start.elapsed();

    let response_log = ResponseLog {
        status_code: response.status().as_u16(),
        duration_ms: elapsed.as_millis() as u64,
        size_bytes: 0, // Would calculate actual size in production
    };

    // Log response
    #[cfg(feature = "logging")]
    tracing::info!(
        method = %request_log.method,
        path = %request_log.path,
        status = response_log.status_code,
        duration_ms = response_log.duration_ms,
        "HTTP request completed"
    );

    Ok(response)
}

/// Create logging middleware with config
pub fn make_logging_middleware(
    config: LoggingConfig,
) -> impl Fn(Request, Next) -> futures::future::BoxFuture<'static, Result<Response, StatusCode>> + Clone
{
    move |req: Request, next: Next| {
        let config = config.clone();
        Box::pin(logging_middleware(req, next, config))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logging_config_default() {
        let config = LoggingConfig::default();
        assert!(config.log_headers);
        assert!(!config.log_body);
        assert!(!config.log_response_body);
    }

    #[test]
    fn test_logging_config_should_log() {
        let config = LoggingConfig::default();
        assert!(config.should_log("/api/users"));
        assert!(!config.should_log("/health"));
        assert!(!config.should_log("/metrics"));
    }

    #[test]
    fn test_logging_config_exclude_path() {
        let config = LoggingConfig::default().exclude_path("/custom");
        assert!(!config.should_log("/custom"));
    }

    #[test]
    fn test_request_log_creation() {
        let log = RequestLog {
            method: "GET".to_string(),
            path: "/api/users".to_string(),
            request_id: Some("req-123".to_string()),
            client_ip: Some("127.0.0.1".to_string()),
            user_agent: Some("test-client".to_string()),
        };
        assert_eq!(log.method, "GET");
        assert_eq!(log.path, "/api/users");
    }
}
