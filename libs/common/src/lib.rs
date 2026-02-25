//! Common library for shared types across all services
//!
//! Contains shared value objects, HTTP utilities, middleware, validation,
//! security utilities, and more.
//!
//! ## Modules
//!
//! - `http` - HTTP utilities (requires `http` feature)
//! - `middleware` - Axum middleware (requires `http` feature)
//! - `extractors` - Request extractors (requires `http` feature)
//! - `value_objects` - Shared value objects (Email, Phone, Money, etc.)
//! - `validation` - Validation rules and request validation
//! - `security` - Security utilities (hashing, secrets, CSRF)
//! - `utils` - General utilities
//! - `observability` - Observability utilities (metrics, tracing)
//! - `time` - Time utilities

// HTTP module - requires axum
#[cfg(feature = "http")]
pub mod http;

// Middleware module - requires axum
#[cfg(feature = "http")]
pub mod middleware;

// Extractors module - requires axum
#[cfg(feature = "http")]
pub mod extractors;

// Value objects module
pub mod value_objects;

// Validation module
pub mod validation;

// Security module
pub mod security;

// Utils module
pub mod utils;

// Observability module
pub mod observability;

// Time module
pub mod time;
