//! Security-related value objects
//!
//! This module contains value objects for security-sensitive data like password hashes and secrets.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Password hash wrapper for secure storage
///
/// Represents a hashed password - the hash itself, not the plaintext.
/// Never contains the original password.
///
/// # Example
///
/// ```rust
/// use common::value_objects::PasswordHash;
///
/// let hash = PasswordHash::new("$argon2id$...");
/// assert_eq!(hash.as_str().len() > 0, true);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasswordHash(pub String);

impl PasswordHash {
    /// Create from a hash string
    ///
    /// This should only be called with verified hash strings (e.g., from DB or fresh hash).
    pub fn new(hash: impl Into<String>) -> Self {
        Self(hash.into())
    }

    /// Get the hash value
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Check if this is a valid hash format (basic check)
    pub fn is_valid(&self) -> bool {
        !self.0.is_empty() && self.0.len() > 20
    }
}

impl fmt::Display for PasswordHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Never display the full hash for security
        write!(f, "[REDACTED]")
    }
}

/// Secret wrapper for sensitive data like API keys and tokens
///
/// Stores sensitive strings with redacted display.
/// Use this for API keys, tokens, and other secrets that should not be logged.
///
/// # Example
///
/// ```rust
/// use common::value_objects::Secret;
///
/// let secret = Secret::new("my-secret-key-12345");
/// // Printing will show [REDACTED]
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Secret(pub String);

impl Secret {
    /// Create a new secret from a string
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Get the secret value (use carefully!)
    ///
    /// This is intentionally not named `as_str` to emphasize caution in usage.
    /// Only call when you actually need the secret value.
    pub fn expose(&self) -> &str {
        &self.0
    }

    /// Check if secret is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get the length of the secret
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl fmt::Display for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[REDACTED]")
    }
}

/// API Key value object
///
/// Type-safe wrapper for API keys with validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApiKey(pub String);

impl ApiKey {
    /// Create from a key string
    pub fn new(key: impl Into<String>) -> Self {
        Self(key.into())
    }

    /// Validate API key format (basic check for minimum length)
    pub fn is_valid(&self) -> bool {
        self.0.len() >= 32 // Arbitrary but reasonable minimum
    }

    /// Get the key value (intentionally clear that this is sensitive)
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ApiKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[REDACTED]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hash_display() {
        let hash = PasswordHash::new("$argon2id$v=19$m=19456,t=2,p=1$...");
        assert_eq!(hash.to_string(), "[REDACTED]");
    }

    #[test]
    fn test_secret_display() {
        let secret = Secret::new("my-secret");
        assert_eq!(secret.to_string(), "[REDACTED]");
    }

    #[test]
    fn test_secret_expose() {
        let secret = Secret::new("test-value");
        assert_eq!(secret.expose(), "test-value");
    }

    #[test]
    fn test_api_key_validation() {
        let short_key = ApiKey::new("short");
        assert!(!short_key.is_valid());

        let long_key = ApiKey::new("a".repeat(35));
        assert!(long_key.is_valid());
    }
}
