//! Core error types and domain errors
//!
//! This module contains the core application error types that are not tied to
//! any specific transport layer (HTTP, etc.). These are pure domain errors.

pub mod app_error;
pub mod codes;
pub mod context;
pub mod kinds;

// Re-export core types for convenience
pub use app_error::{AppError, AppResult};
pub use codes::{auth_error::AuthErrorCode};
pub use context::{ContextualError, ErrorContext};
