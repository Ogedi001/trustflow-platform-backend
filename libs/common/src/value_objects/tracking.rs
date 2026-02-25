//! Tracking value objects for distributed tracing
//!
//! This module provides unified value objects for request tracking across services:
//! - `RequestId`: Unique identifier for each request
//! - `CorrelationId`: ID for tracing requests across service boundaries
//! - `IdempotencyKey`: Key for idempotent operation detection
//!
//! Header constants are defined in `http::headers::constants`

use std::fmt;
use uuid::Uuid;

/// Request ID for tracing individual requests
///
/// A RequestId is a unique identifier generated for each incoming request.
/// It can be either a UUID (generated) or a provided string (from upstream).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RequestId(String);

impl RequestId {
    /// Create a new RequestId with a randomly generated UUID
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Create a RequestId from an existing string value
    pub fn from_string(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Parse a RequestId from a string, validating UUID format
    ///
    /// Returns `None` if the string is not a valid UUID
    pub fn parse(value: &str) -> Option<Self> {
        if Uuid::try_parse(value).is_ok() {
            Some(Self(value.to_string()))
        } else {
            None
        }
    }

    /// Get the underlying string representation
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Check if the RequestId is a valid UUID
    pub fn is_valid_uuid(&self) -> bool {
        Uuid::parse_str(&self.0).is_ok()
    }
}

impl Default for RequestId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for RequestId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<RequestId> for String {
    fn from(id: RequestId) -> Self {
        id.0
    }
}

/// Correlation ID for distributed tracing
///
/// A CorrelationId is used to track a request across multiple service boundaries.
/// It is typically propagated from upstream services or generated if not present.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CorrelationId(String);

impl CorrelationId {
    /// Create a new CorrelationId with a randomly generated UUID
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Create a CorrelationId from an existing string value
    pub fn from_string(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Parse a CorrelationId from a string, validating UUID format
    pub fn parse(value: &str) -> Option<Self> {
        if Uuid::try_parse(value).is_ok() {
            Some(Self(value.to_string()))
        } else {
            None
        }
    }

    /// Get the underlying string representation
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Check if the CorrelationId is a valid UUID
    pub fn is_valid_uuid(&self) -> bool {
        Uuid::parse_str(&self.0).is_ok()
    }
}

impl Default for CorrelationId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CorrelationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<CorrelationId> for String {
    fn from(id: CorrelationId) -> Self {
        id.0
    }
}

/// Idempotency key for preventing duplicate operations
///
/// An IdempotencyKey is used to identify duplicate requests for operations
/// that should only be executed once (e.g., payment processing).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdempotencyKey(String);

impl IdempotencyKey {
    /// Create a new IdempotencyKey
    pub fn new(key: impl Into<String>) -> Self {
        Self(key.into())
    }

    /// Get the underlying string representation
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Check if the key is valid (non-empty)
    pub fn is_valid(&self) -> bool {
        !self.0.is_empty()
    }
}

impl fmt::Display for IdempotencyKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<IdempotencyKey> for String {
    fn from(key: IdempotencyKey) -> Self {
        key.0
    }
}

/// Combined tracking context containing all tracking identifiers
#[derive(Debug, Clone)]
pub struct TrackingContext {
    pub request_id: RequestId,
    pub correlation_id: CorrelationId,
    pub idempotency_key: Option<IdempotencyKey>,
}

impl TrackingContext {
    /// Create a new tracking context with generated IDs
    pub fn new() -> Self {
        Self {
            request_id: RequestId::new(),
            correlation_id: CorrelationId::new(),
            idempotency_key: None,
        }
    }

    /// Create tracking context from request headers
    pub fn from_headers(
        request_id: Option<String>,
        correlation_id: Option<String>,
        idempotency_key: Option<String>,
    ) -> Self {
        Self {
            request_id: request_id
                .and_then(|v| RequestId::parse(&v))
                .unwrap_or_else(RequestId::new),
            correlation_id: correlation_id
                .and_then(|v| CorrelationId::parse(&v))
                .unwrap_or_else(CorrelationId::new),
            idempotency_key: idempotency_key.map(IdempotencyKey::new),
        }
    }

    /// Set idempotency key
    pub fn with_idempotency_key(mut self, key: IdempotencyKey) -> Self {
        self.idempotency_key = Some(key);
        self
    }
}

impl Default for TrackingContext {
    fn default() -> Self {
        Self::new()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_request_id_generation() {
//         let id = RequestId::new();
//         assert!(id.is_valid_uuid());
//         assert!(!id.as_str().is_empty());
//     }

//     #[test]
//     fn test_request_id_parse_valid() {
//         let uuid = "550e8400-e29b-41d4-a716-446655440000";
//         let id = RequestId::parse(uuid);
//         assert!(id.is_some());
//         assert_eq!(id.unwrap().as_str(), uuid);
//     }

//     #[test]
//     fn test_request_id_parse_invalid() {
//         let id = RequestId::parse("not-a-uuid");
//         assert!(id.is_none());
//     }

//     #[test]
//     fn test_correlation_id_generation() {
//         let id = CorrelationId::new();
//         assert!(id.is_valid_uuid());
//     }

//     #[test]
//     fn test_idempotency_key() {
//         let key = IdempotencyKey::new("unique-key-123");
//         assert!(key.is_valid());
//         assert_eq!(key.as_str(), "unique-key-123");
//     }

//     #[test]
//     fn test_tracking_context() {
//         let ctx = TrackingContext::new();
//         assert!(ctx.request_id.is_valid_uuid());
//         assert!(ctx.correlation_id.is_valid_uuid());
//         assert!(ctx.idempotency_key.is_none());
//     }
// }
