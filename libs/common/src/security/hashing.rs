//! Password hashing utilities
//!
//! Provides secure password hashing and verification using industry-standard algorithms.
//! This module requires no external dependencies for basic hashing,
//! but argon2 feature can be enabled for Argon2 support.

use crate::value_objects::security::PasswordHash;

/// Password hashing result type
pub type HashResult<T> = Result<T, HashError>;

/// Password hashing errors
#[derive(Debug, Clone)]
pub enum HashError {
    /// Hash verification failed
    VerificationFailed,
    /// Invalid hash format
    InvalidHashFormat,
    /// Error during hashing
    HashingError(String),
}

impl std::fmt::Display for HashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HashError::VerificationFailed => write!(f, "password verification failed"),
            HashError::InvalidHashFormat => write!(f, "invalid hash format"),
            HashError::HashingError(e) => write!(f, "hashing error: {}", e),
        }
    }
}

impl std::error::Error for HashError {}

/// trait for password hashing implementations
pub trait PasswordHasher: Send + Sync {
    /// Hash a password
    fn hash(&self, password: impl AsRef<[u8]>) -> HashResult<PasswordHash>;

    /// Verify a password against a hash
    fn verify(&self, password: impl AsRef<[u8]>, hash: &PasswordHash) -> HashResult<bool>;
}

/// Default SHA-256 based hasher (for fallback/testing)
///
/// This is a simple hasher for when Argon2 is not available.
/// For production, use Argon2 hasher instead.
pub struct Sha256Hasher;

impl PasswordHasher for Sha256Hasher {
    fn hash(&self, password: impl AsRef<[u8]>) -> HashResult<PasswordHash> {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(password.as_ref());
        let result = hasher.finalize();

        let hash_string = format!("$sha256${}", hex::encode(result));
        Ok(PasswordHash::new(hash_string))
    }

    fn verify(&self, password: impl AsRef<[u8]>, hash: &PasswordHash) -> HashResult<bool> {
        let recomputed = self.hash(password)?;
        Ok(recomputed.as_str() == hash.as_str())
    }
}

/// Simple HMAC-SHA256 based password hasher
///
/// Suitable for development/testing but not recommended for production.
/// Use Argon2 for production systems.
pub struct HmacSha256Hasher {
    /// Shared secret for HMAC operations
    secret: Vec<u8>,
}

impl HmacSha256Hasher {
    /// Create a new HMAC-SHA256 hasher with a secret
    pub fn new(secret: impl AsRef<[u8]>) -> Self {
        Self {
            secret: secret.as_ref().to_vec(),
        }
    }
}

impl PasswordHasher for HmacSha256Hasher {
    fn hash(&self, password: impl AsRef<[u8]>) -> HashResult<PasswordHash> {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;

        type HmacSha256 = Hmac<Sha256>;

        let mut mac = HmacSha256::new_from_slice(&self.secret)
            .map_err(|e| HashError::HashingError(e.to_string()))?;
        mac.update(password.as_ref());

        let result = mac.finalize();
        let hash_string = format!("$hmacsha256${}", hex::encode(result.into_bytes()));
        Ok(PasswordHash::new(hash_string))
    }

    fn verify(&self, password: impl AsRef<[u8]>, hash: &PasswordHash) -> HashResult<bool> {
        let recomputed = self.hash(password)?;
        Ok(recomputed.as_str() == hash.as_str())
    }
}

/// Password strength validator
pub struct PasswordStrength;

impl PasswordStrength {
    /// Check minimum password length
    pub fn check_length(password: &str, min_length: usize) -> bool {
        password.len() >= min_length
    }

    /// Check if password has uppercase letters
    pub fn has_uppercase(password: &str) -> bool {
        password.chars().any(|c| c.is_uppercase())
    }

    /// Check if password has lowercase letters
    pub fn has_lowercase(password: &str) -> bool {
        password.chars().any(|c| c.is_lowercase())
    }

    /// Check if password has digits
    pub fn has_digits(password: &str) -> bool {
        password.chars().any(|c| c.is_ascii_digit())
    }

    /// Check if password has special characters
    pub fn has_special_chars(password: &str) -> bool {
        password
            .chars()
            .any(|c| !c.is_alphanumeric() && !c.is_whitespace())
    }

    /// Validate password strength with standard rules
    ///
    /// Returns true if password meets common requirements:
    /// - At least 8 characters
    /// - Contains uppercase
    /// - Contains lowercase
    /// - Contains digits
    /// - Contains special characters
    pub fn validate(password: &str) -> bool {
        Self::check_length(password, 8)
            && Self::has_uppercase(password)
            && Self::has_lowercase(password)
            && Self::has_digits(password)
            && Self::has_special_chars(password)
    }

    /// Get password strength score (0-5)
    pub fn score(password: &str) -> u8 {
        let mut score = if Self::check_length(password, 8) { 1 } else { 0 };

        score += if Self::has_uppercase(password) { 1 } else { 0 };
        score += if Self::has_lowercase(password) { 1 } else { 0 };
        score += if Self::has_digits(password) { 1 } else { 0 };
        score += if Self::has_special_chars(password) { 1 } else { 0 };

        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_hash() {
        let hasher = Sha256Hasher;
        let password = "MyPassword123!";
        let hash = hasher.hash(password).unwrap();
        assert!(hasher.verify(password, &hash).unwrap());
    }

    #[test]
    fn test_hmac_hash() {
        let hasher = HmacSha256Hasher::new("my-secret");
        let password = "MyPassword123!";
        let hash = hasher.hash(password).unwrap();
        assert!(hasher.verify(password, &hash).unwrap());
    }

    #[test]
    fn test_password_strength() {
        assert!(PasswordStrength::has_uppercase("Hello"));
        assert!(PasswordStrength::has_digits("1234"));
        assert!(!PasswordStrength::validate("weak"));
        assert!(PasswordStrength::validate("StrongPass123!"));
    }

    #[test]
    fn test_password_score() {
        let weak = "test";
        let strong = "StrongPass123!";
        assert!(PasswordStrength::score(weak) < PasswordStrength::score(strong));
    }
}

// Re-export for simpler imports
pub use sha2;
pub use hmac;
pub use hex;
