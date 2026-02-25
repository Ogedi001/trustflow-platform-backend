//! HTTP header utilities and helpers

use axum::http::HeaderMap;
use std::str::FromStr;

/// Standard HTTP headers for request/response tracking
pub struct TrackingHeaders {
    /// Unique request ID
    pub request_id: Option<String>,
    /// Correlation ID for tracing across services
    pub correlation_id: Option<String>,
    /// Idempotency key for idempotent operations
    pub idempotency_key: Option<String>,
}

impl TrackingHeaders {
    /// Extract tracking headers from request header map
    pub fn from_headers(headers: &HeaderMap) -> Self {
        let request_id = headers
            .get("x-request-id")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let correlation_id = headers
            .get("x-correlation-id")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let idempotency_key = headers
            .get("idempotency-key")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        Self {
            request_id,
            correlation_id,
            idempotency_key,
        }
    }
}

/// Helper for building HTTP headers
pub struct HeaderBuilder {
    headers: HeaderMap,
}

impl HeaderBuilder {
    /// Create new header builder
    pub fn new() -> Self {
        Self {
            headers: HeaderMap::new(),
        }
    }

    /// Add a header
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let key = key.into();
        let value = value.into();
        if let Ok(name) = axum::http::HeaderName::from_str(&key) {
            if let Ok(val) = axum::http::HeaderValue::from_str(&value) {
                self.headers.insert(name, val);
            }
        }
        self
    }

    /// Add request ID
    pub fn request_id(self, id: impl Into<String>) -> Self {
        self.header("x-request-id", id)
    }

    /// Add correlation ID
    pub fn correlation_id(self, id: impl Into<String>) -> Self {
        self.header("x-correlation-id", id)
    }

    /// Add idempotency key
    pub fn idempotency_key(self, key: impl Into<String>) -> Self {
        self.header("idempotency-key", key)
    }

    /// Add custom JSON content type
    pub fn json_content_type(self) -> Self {
        self.header("content-type", "application/json")
    }

    /// Add cache control
    pub fn cache_control(self, directive: impl Into<String>) -> Self {
        self.header("cache-control", directive)
    }

    /// Add CORS origin
    pub fn cors_origin(self, origin: impl Into<String>) -> Self {
        self.header("access-control-allow-origin", origin)
    }

    /// Build header map
    pub fn build(self) -> HeaderMap {
        self.headers
    }
}

impl Default for HeaderBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Common header constants
pub mod constants {
    pub const REQUEST_ID: &str = "x-request-id";
    pub const CORRELATION_ID: &str = "x-correlation-id";
    pub const IDEMPOTENCY_KEY: &str = "idempotency-key";
    pub const RATE_LIMIT_LIMIT: &str = "x-ratelimit-limit";
    pub const RATE_LIMIT_REMAINING: &str = "x-ratelimit-remaining";
    pub const RATE_LIMIT_RESET: &str = "x-ratelimit-reset";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracking_headers_extraction() {
        let mut headers = HeaderMap::new();
        headers.insert("x-request-id", "req-123".parse().unwrap());
        headers.insert("x-correlation-id", "corr-456".parse().unwrap());

        let tracking = TrackingHeaders::from_headers(&headers);
        assert_eq!(tracking.request_id, Some("req-123".to_string()));
        assert_eq!(tracking.correlation_id, Some("corr-456".to_string()));
    }

    #[test]
    fn test_header_builder_builds_headers() {
        let headers = HeaderBuilder::new()
            .request_id("req-789")
            .correlation_id("corr-012")
            .json_content_type()
            .build();

        assert!(headers.contains_key("x-request-id"));
        assert!(headers.contains_key("x-correlation-id"));
        assert!(headers.contains_key("content-type"));
    }
}
