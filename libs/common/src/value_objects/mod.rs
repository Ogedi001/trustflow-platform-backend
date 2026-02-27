//! Value objects module
//!
//! This module contains domain-driven value objects that represent core concepts
//! used across all services. Value objects are immutable, type-safe wrappers around
//! primitive types to provide semantic meaning and validation.
//!
//! ## Organization
//!
//! - `identity` - User and resource identifiers (UserId, ResourceId, DeviceId)
//! - `contact` - Contact information (EmailAddress, PhoneNumber)
//! - `security` - Security-related objects (PasswordHash, Secret, ApiKey)
//! - `network` - Network identifiers (Url, IpAddress, UserAgent)
//! - `pagination_vo` - Query pagination and sorting (Pagination, Sort, SearchParams)
//! - `timestamps` - Time-related objects (Timestamp, Duration, TimeRange)
//! - `tracking` - Request tracking (RequestId, CorrelationId, IdempotencyKey)
//! - `ulid` - ULID identifiers
//!
//! ## Features
//!
//! All value objects are:
//! - Immutable for thread safety
//! - Serializable with serde
//! - Type-safe with newtype patterns
//! - Validated at construction time

pub mod contact;
pub mod core;
pub mod identity;
pub mod network;
pub mod pagination_vo;
pub mod security;
pub mod timestamps;
pub mod tracking;
// pub mod email;
// pub mod phone;
pub mod ulid;

// Re-export all tracking types from unified tracking module
pub use contact::{EmailAddress, PhoneNumber};
pub use security::{ApiKey, PasswordHash, Secret};
pub use timestamps::{Duration, TimeRange, Timestamp};
pub use tracking::{CorrelationId, IdempotencyKey, RequestId, TrackingContext};
