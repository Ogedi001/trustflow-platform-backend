//! CSRF token generation and validation
//!
//! Provides utilities for CSRF (Cross-Site Request Forgery) token generation
//! and validation for HTTP endpoints.

use crate::value_objects::security::Secret;
use fastrand;

/// CSRF token wrapper
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsrfToken(String);

impl CsrfToken {
    /// Create from a token string
    pub fn new(token: String) -> Self {
        Self(token)
    }

    /// Get the token value
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Get as Secret for safe storage/transmission
    pub fn to_secret(&self) -> Secret {
        Secret::new(self.0.clone())
    }
}

impl std::fmt::Display for CsrfToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Display should show [REDACTED] for safety
        write!(f, "[REDACTED]")
    }
}

/// CSRF token generator
pub struct CsrfGenerator;

impl CsrfGenerator {
    /// Generate a secure random CSRF token (32 bytes = 256 bits)
    pub fn generate() -> CsrfToken {
        let random_bytes = (0..32)
            .map(|_| fastrand::u8(0..=255))
            .collect::<Vec<_>>();
        let token = hex::encode(random_bytes);
        CsrfToken::new(token)
    }

    /// Generate a token with custom length (in bytes)
    pub fn generate_with_length(length: usize) -> CsrfToken {
        let random_bytes = (0..length)
            .map(|_| fastrand::u8(0..=255))
            .collect::<Vec<_>>();
        let token = hex::encode(random_bytes);
        CsrfToken::new(token)
    }

    /// Validate token format (basic check)
    pub fn is_valid_format(token: &str) -> bool {
        // Should be hex-encoded, even length, at least 32 chars (16 bytes minimum)
        token.len() >= 32
            && token.len() % 2 == 0
            && token.chars().all(|c| c.is_ascii_hexdigit())
    }
}

/// CSRF validator for double-submit cookie pattern
pub struct CsrfValidator {
    session_token: String,
}

impl CsrfValidator {
    /// Create a validator with a stored session token
    pub fn new(session_token: impl Into<String>) -> Self {
        Self {
            session_token: session_token.into(),
        }
    }

    /// Verify that request token matches session token
    pub fn verify(&self, request_token: &str) -> bool {
        // Constant-time comparison to prevent timing attacks
        if self.session_token.len() != request_token.len() {
            return false;
        }

        self.session_token
            .as_bytes()
            .iter()
            .zip(request_token.as_bytes())
            .fold(0u8, |acc, (a, b)| acc | (a ^ b))
            == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_token() {
        let token = CsrfGenerator::generate();
        assert!(CsrfGenerator::is_valid_format(token.as_str()));

        let token2 = CsrfGenerator::generate();
        // Tokens should be different (astronomically unlikely to be the same)
        assert_ne!(token.as_str(), token2.as_str());
    }

    #[test]
    fn test_generate_custom_length() {
        let token = CsrfGenerator::generate_with_length(16);
        assert!(token.as_str().len() >= 32);
    }

    #[test]
    fn test_token_validation() {
        let valid = "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4";
        assert!(CsrfGenerator::is_valid_format(valid));

        let invalid_short = "short";
        assert!(!CsrfGenerator::is_valid_format(invalid_short));

        let invalid_chars = "gggggggggggggggggggggggggggggggg";
        assert!(!CsrfGenerator::is_valid_format(invalid_chars));
    }

    #[test]
    fn test_csrf_validator() {
        let token = CsrfGenerator::generate();
        let validator = CsrfValidator::new(token.as_str());

        assert!(validator.verify(token.as_str()));
        assert!(!validator.verify("wrongtoken123456789012345678901"));
    }

    #[test]
    fn test_token_display() {
        let token = CsrfGenerator::generate();
        assert_eq!(token.to_string(), "[REDACTED]");
    }
}
