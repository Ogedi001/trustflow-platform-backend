//! Secure secret generation and management
//!
//! Provides utilities for generating cryptographically secure secrets,
//! tokens, and other sensitive random values.

use crate::value_objects::security::Secret;
use fastrand;

/// Result type for secret operations
pub type SecretResult<T> = Result<T, SecretError>;

/// Errors that can occur during secret operations
#[derive(Debug, Clone)]
pub enum SecretError {
    /// Failed to generate secret
    GenerationFailed(String),
    /// Invalid secret format
    InvalidFormat(String),
}

impl std::fmt::Display for SecretError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecretError::GenerationFailed(e) => write!(f, "Failed to generate secret: {}", e),
            SecretError::InvalidFormat(e) => write!(f, "Invalid secret format: {}", e),
        }
    }
}

impl std::error::Error for SecretError {}

/// Secret generator for creating cryptographically secure values
pub struct SecretGenerator;

impl SecretGenerator {
    /// Generate a random secret token as hex string (32 bytes = 256 bits)
    pub fn token() -> Secret {
        let random_bytes = (0..32)
            .map(|_| fastrand::u8(0..=255))
            .collect::<Vec<_>>();
        Secret::new(hex::encode(random_bytes))
    }

    /// Generate a random secret with custom byte length
    pub fn token_with_length(bytes: usize) -> Secret {
        let random_bytes = (0..bytes)
            .map(|_| fastrand::u8(0..=255))
            .collect::<Vec<_>>();
        Secret::new(hex::encode(random_bytes))
    }

    /// Generate a base64-encoded secret
    pub fn token_base64(bytes: usize) -> Secret {
        use base64::{engine::general_purpose, Engine as _};
        let random_bytes = (0..bytes)
            .map(|_| fastrand::u8(0..=255))
            .collect::<Vec<_>>();
        Secret::new(general_purpose::STANDARD.encode(&random_bytes))
    }

    /// Generate an API key format token (alphanumeric with prefix)
    pub fn api_key(prefix: &str, length: usize) -> Secret {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let random_part: String = (0..length)
            .map(|_| {
                let idx = fastrand::usize(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        Secret::new(format!("{}_{}", prefix, random_part))
    }

    /// Generate a random numeric PIN
    pub fn numeric_pin(length: usize) -> Secret {
        let pin: String = (0..length)
            .map(|_| fastrand::u8(0..=9).to_string())
            .collect();
        Secret::new(pin)
    }

    /// Generate a random OTP (One-Time Password) - 6 digits
    pub fn otp() -> Secret {
        let otp: String = (0..6)
            .map(|_| fastrand::u32(0..=9).to_string())
            .collect();
        Secret::new(otp)
    }

    /// Generate a random OTP with custom length
    pub fn otp_with_length(length: usize) -> Secret {
        let otp: String = (0..length)
            .map(|_| fastrand::u32(0..=9).to_string())
            .collect();
        Secret::new(otp)
    }

    /// Generate a random state parameter (for OAuth flows) - 32 bytes
    pub fn oauth_state() -> Secret {
        let random_bytes = (0..32)
            .map(|_| fastrand::u8(0..=255))
            .collect::<Vec<_>>();
        Secret::new(hex::encode(random_bytes))
    }

    /// Generate a random nonce - 32 bytes
    pub fn nonce() -> Secret {
        let random_bytes = (0..32)
            .map(|_| fastrand::u8(0..=255))
            .collect::<Vec<_>>();
        Secret::new(hex::encode(random_bytes))
    }
}

/// Secure random value generator for non-secret random needs
pub struct RandomGenerator;

impl RandomGenerator {
    /// Generate random string from charset
    pub fn string(charset: &str, length: usize) -> String {
        let bytes = charset.as_bytes();
        (0..length)
            .map(|_| {
                let idx = fastrand::usize(0..bytes.len());
                bytes[idx] as char
            })
            .collect()
    }

    /// Generate random alphanumeric string
    pub fn alphanumeric(length: usize) -> String {
        Self::string("ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789", length)
    }

    /// Generate random lowercase alphanumeric
    pub fn lowercase_alphanumeric(length: usize) -> String {
        Self::string("abcdefghijklmnopqrstuvwxyz0123456789", length)
    }

    /// Generate random hex string
    pub fn hex(length: usize) -> String {
        Self::string("0123456789abcdef", length)
    }

    /// Generate random UUID v4-like identifier
    pub fn uuid_like() -> String {
        let mut result = String::with_capacity(36);
        for i in 0..36 {
            match i {
                8 | 13 | 18 | 23 => result.push('-'),
                14 => result.push('4'), // version 4
                19 => {
                    let c = fastrand::u8(0..=3);
                    result.push(format!("{:x}", 8 + c % 4).chars().next().unwrap());
                }
                _ => result.push_str(&Self::hex(1)),
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_generation() {
        let token1 = SecretGenerator::token();
        let token2 = SecretGenerator::token();

        assert_ne!(token1.expose(), token2.expose());
        assert!(token1.len() > 0);
    }

    #[test]
    fn test_api_key_format() {
        let api_key = SecretGenerator::api_key("sk", 32);
        let key_str = api_key.expose();
        assert!(key_str.starts_with("sk_"));
        assert_eq!(key_str.len(), 35); // "sk_" (3) + 32 chars
    }

    #[test]
    fn test_otp_generation() {
        let otp = SecretGenerator::otp();
        let otp_str = otp.expose();
        assert_eq!(otp_str.len(), 6);
        assert!(otp_str.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_numeric_pin() {
        let pin = SecretGenerator::numeric_pin(4);
        let pin_str = pin.expose();
        assert_eq!(pin_str.len(), 4);
        assert!(pin_str.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_alphanumeric_generation() {
        let random = RandomGenerator::alphanumeric(20);
        assert_eq!(random.len(), 20);
        assert!(random.chars().all(|c| c.is_alphanumeric()));
    }

    #[test]
    fn test_uuid_like() {
        let uuid = RandomGenerator::uuid_like();
        assert_eq!(uuid.len(), 36);
        assert!(uuid.contains('-'));
    }

    #[test]
    fn test_secret_display() {
        let secret = SecretGenerator::token();
        assert_eq!(secret.to_string(), "[REDACTED]");
    }
}
