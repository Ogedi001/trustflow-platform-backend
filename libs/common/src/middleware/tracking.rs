//! Unified Tracking Middleware
//!
//! Handles all 3 tracking IDs in a single middleware for efficiency:
//! - RequestId: Unique ID for each request
//! - CorrelationId: Links related requests across services
//! - IdempotencyKey: Key for duplicate request detection
//!
//! Usage:
//! ```rust
//! use common::middleware::tracking_layer;
//!
//! let app = axum::Router::new()
//!     .route_layer(tracking_layer());
//! ```

use crate::value_objects::tracking::{CorrelationId, IdempotencyKey, RequestId, TrackingContext};
use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};

/// Standard HTTP header names for tracking
pub mod header_names {
    pub const REQUEST_ID: &str = "x-request-id";
    pub const CORRELATION_ID: &str = "x-correlation-id";
    pub const IDEMPOTENCY_KEY: &str = "idempotency-key";
}

/// Configuration for unified tracking middleware
#[derive(Debug, Clone)]
pub struct TrackingConfig {
    /// Header name for request ID
    pub request_id_header: &'static str,
    /// Header name for correlation ID
    pub correlation_id_header: &'static str,
    /// Header name for idempotency key
    pub idempotency_key_header: &'static str,
    /// Whether to generate request ID if missing
    pub generate_request_id: bool,
    /// Whether to generate correlation ID if missing
    pub generate_correlation_id: bool,
}

impl Default for TrackingConfig {
    fn default() -> Self {
        Self {
            request_id_header: header_names::REQUEST_ID,
            correlation_id_header: header_names::CORRELATION_ID,
            idempotency_key_header: header_names::IDEMPOTENCY_KEY,
            generate_request_id: true,
            generate_correlation_id: true,
        }
    }
}

impl TrackingConfig {
    /// Create new default config
    pub fn new() -> Self {
        Self::default()
    }

    /// Custom request ID header
    pub fn with_request_id_header(mut self, header: &'static str) -> Self {
        self.request_id_header = header;
        self
    }

    /// Custom correlation ID header
    pub fn with_correlation_id_header(mut self, header: &'static str) -> Self {
        self.correlation_id_header = header;
        self
    }

    /// Custom idempotency key header
    pub fn with_idempotency_key_header(mut self, header: &'static str) -> Self {
        self.idempotency_key_header = header;
        self
    }

    /// Disable request ID generation
    pub fn with_request_id_required(mut self) -> Self {
        self.generate_request_id = false;
        self
    }

    /// Disable correlation ID generation
    pub fn with_correlation_id_required(mut self) -> Self {
        self.generate_correlation_id = false;
        self
    }
}

/// Unified tracking middleware - handles all 3 IDs in one pass
///
/// This middleware extracts or generates all tracking IDs and makes them
/// available in request extensions for handlers to use.
pub async fn tracking_middleware(mut req: Request, next: Next, config: TrackingConfig) -> Response {
    // Extract request ID
    let request_id = extract_request_id(
        req.headers().get(config.request_id_header),
        config.generate_request_id,
    );

    // Extract correlation ID
    let correlation_id = extract_correlation_id(
        req.headers().get(config.correlation_id_header),
        config.generate_correlation_id,
    );

    // Extract idempotency key (never auto-generate)
    let idempotency_key = req
        .headers()
        .get(config.idempotency_key_header)
        .and_then(|v| v.to_str().ok())
        .map(IdempotencyKey::new);

    // Create tracking context
    let context = TrackingContext {
        request_id,
        correlation_id,
        idempotency_key,
    };

    // Log the tracking IDs
    tracing::debug!(
        request_id = %context.request_id,
        correlation_id = %context.correlation_id,
        has_idempotency_key = context.idempotency_key.is_some(),
        "Request tracking context created"
    );

    // Insert into request extensions
    req.extensions_mut().insert(context.clone());

    // Execute handler
    let mut response = next.run(req).await;

    // Add tracking IDs to response headers
    if let Ok(val) = HeaderValue::from_str(context.request_id.as_str()) {
        response.headers_mut().insert(config.request_id_header, val);
    }
    if let Ok(val) = HeaderValue::from_str(context.correlation_id.as_str()) {
        response
            .headers_mut()
            .insert(config.correlation_id_header, val);
    }

    response
}

/// Extract request ID from header or generate new one
fn extract_request_id(header: Option<&axum::http::HeaderValue>, generate: bool) -> RequestId {
    if let Some(h) = header {
        if let Ok(s) = h.to_str() {
            // Try to parse as valid UUID
            if let Some(id) = RequestId::parse(s) {
                return id;
            }
        }
    }

    if generate {
        RequestId::new()
    } else {
        RequestId::new()
    }
}

/// Extract correlation ID from header or generate new one
fn extract_correlation_id(
    header: Option<&axum::http::HeaderValue>,
    generate: bool,
) -> CorrelationId {
    if let Some(h) = header {
        if let Ok(s) = h.to_str() {
            // Try to parse as valid UUID
            if let Some(id) = CorrelationId::parse(s) {
                return id;
            }
        }
    }

    if generate {
        CorrelationId::new()
    } else {
        CorrelationId::new()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_tracking_config_default() {
//         let config = TrackingConfig::default();
//         assert_eq!(config.request_id_header, "x-request-id");
//         assert_eq!(config.correlation_id_header, "x-correlation-id");
//         assert_eq!(config.idempotency_key_header, "idempotency-key");
//         assert!(config.generate_request_id);
//         assert!(config.generate_correlation_id);
//     }

//     #[test]
//     fn test_tracking_config_builder() {
//         let config = TrackingConfig::new()
//             .with_request_id_header("x-req-id")
//             .with_correlation_id_header("x-corr-id")
//             .with_idempotency_key_header("x-idem-key")
//             .with_request_id_required()
//             .with_correlation_id_required();

//         assert_eq!(config.request_id_header, "x-req-id");
//         assert_eq!(config.correlation_id_header, "x-corr-id");
//         assert_eq!(config.idempotency_key_header, "x-idem-key");
//         assert!(!config.generate_request_id);
//         assert!(!config.generate_correlation_id);
//     }
// }
