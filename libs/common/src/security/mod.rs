//! Security module
//!
//! This module provides security utilities for password hashing, secret generation,
//! and CSRF token management.
//!
//! ## Organization
//!
//! - `hashing` - Password hashing and strength validation
//! - `csrf` - CSRF token generation and validation
//! - `secrets` - Cryptographically secure random generation
//!
//! ## Quick Start
//!
//! ```rust
//! use common::security::{CsrfGenerator, SecretGenerator, PasswordStrength};
//!
//! // Generate CSRF token
//! let csrf_token = CsrfGenerator::generate();
//!
//! // Generate API key
//! let api_key = SecretGenerator::api_key("sk", 32);
//!
//! // Check password strength
//! let is_strong = PasswordStrength::validate("MySecurePass123!");
//! ```

pub mod csrf;
pub mod hashing;
pub mod secrets;

pub use csrf::{CsrfGenerator, CsrfToken, CsrfValidator};
pub use hashing::{HmacSha256Hasher, PasswordHasher, PasswordStrength, Sha256Hasher};
pub use secrets::{RandomGenerator, SecretGenerator, SecretError, SecretResult};

/// Prelude module for convenient importing
pub mod prelude {
    //! Import common security items with `use common::security::prelude::*;`
    pub use super::{CsrfGenerator, CsrfToken, CsrfValidator, PasswordStrength, RandomGenerator, SecretGenerator};
}
