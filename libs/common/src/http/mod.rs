//! HTTP module for common HTTP utilities
//!
//! This module provides common HTTP utilities used across all services.
//! Available when the `http` feature is enabled.

pub mod error;
pub mod fallback;
pub mod headers;
pub mod health;
pub mod meta;
pub mod pagination;
pub mod response;
