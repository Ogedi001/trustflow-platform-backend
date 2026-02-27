//! MFA (Multi-Factor Authentication) configuration
//!
//! Provides configuration types for MFA functionality in the identity service.

use serde::{Deserialize, Serialize};
use time::Duration;

/// MFA configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaConfig {
    /// Whether MFA is enabled
    pub enabled: bool,
    /// Issuer name for TOTP (e.g., "TrustFlow")
    pub issuer_name: String,
    /// Number of digits for TOTP code
    pub totp_digits: u8,
    /// Time period for TOTP (usually 30 seconds)
    pub totp_period: Duration,
    /// Length of SMS OTP
    pub sms_otp_length: u8,
    /// TTL for SMS OTP (time to live)
    pub sms_otp_ttl: Duration,
    /// Whether to allow email OTP
    pub email_otp_enabled: bool,
    /// Length of email OTP
    pub email_otp_length: u8,
    /// TTL for email OTP
    pub email_otp_ttl: Duration,
    /// Maximum number of MFA devices allowed per user
    pub max_devices_per_user: u32,
    /// Whether to require MFA for all users
    pub required_for_all: bool,
}

impl Default for MfaConfig {
    fn default() -> Self {
        Self::from_env()
    }
}

impl MfaConfig {
    /// Create configuration from environment variables
    pub fn from_env() -> Self {
        Self {
            enabled: std::env::var("MFA_ENABLED")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            issuer_name: std::env::var("MFA_ISSUER_NAME")
                .unwrap_or_else(|_| "TrustFlow".to_string()),
            totp_digits: std::env::var("MFA_TOTP_DIGITS")
                .unwrap_or_else(|_| "6".to_string())
                .parse()
                .unwrap_or(6),
            totp_period: Duration::seconds(
                std::env::var("MFA_TOTP_PERIOD")
                    .unwrap_or_else(|_| "30".to_string())
                    .parse()
                    .unwrap_or(30),
            ),
            sms_otp_length: std::env::var("MFA_SMS_OTP_LENGTH")
                .unwrap_or_else(|_| "6".to_string())
                .parse()
                .unwrap_or(6),
            sms_otp_ttl: Duration::seconds(
                std::env::var("MFA_SMS_OTP_TTL")
                    .unwrap_or_else(|_| "300".to_string()) // 5 minutes
                    .parse()
                    .unwrap_or(300),
            ),
            email_otp_enabled: std::env::var("MFA_EMAIL_OTP_ENABLED")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            email_otp_length: std::env::var("MFA_EMAIL_OTP_LENGTH")
                .unwrap_or_else(|_| "6".to_string())
                .parse()
                .unwrap_or(6),
            email_otp_ttl: Duration::seconds(
                std::env::var("MFA_EMAIL_OTP_TTL")
                    .unwrap_or_else(|_| "600".to_string()) // 10 minutes
                    .parse()
                    .unwrap_or(600),
            ),
            max_devices_per_user: std::env::var("MFA_MAX_DEVICES_PER_USER")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .unwrap_or(5),
            required_for_all: std::env::var("MFA_REQUIRED_FOR_ALL")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
        }
    }

    /// Create configuration from a loader
    pub fn from_loader(loader: &crate::sources::ConfigLoader) -> crate::core::ConfigResult<Self> {
        Ok(Self {
            enabled: loader.get_or("MFA_ENABLED", true)?,
            issuer_name: loader.get_or("MFA_ISSUER_NAME", "TrustFlow".to_string())?,
            totp_digits: loader.get_or("MFA_TOTP_DIGITS", 6u8)?,
            totp_period: Duration::seconds(loader.get_or("MFA_TOTP_PERIOD", 30i64)?),
            sms_otp_length: loader.get_or("MFA_SMS_OTP_LENGTH", 6u8)?,
            sms_otp_ttl: Duration::seconds(loader.get_or("MFA_SMS_OTP_TTL", 300i64)?),
            email_otp_enabled: loader.get_or("MFA_EMAIL_OTP_ENABLED", true)?,
            email_otp_length: loader.get_or("MFA_EMAIL_OTP_LENGTH", 6u8)?,
            email_otp_ttl: Duration::seconds(loader.get_or("MFA_EMAIL_OTP_TTL", 600i64)?),
            max_devices_per_user: loader.get_or("MFA_MAX_DEVICES_PER_USER", 5u32)?,
            required_for_all: loader.get_or("MFA_REQUIRED_FOR_ALL", false)?,
        })
    }

    /// Validate the configuration
    pub fn validate(&self) -> crate::core::ConfigResult<()> {
        if self.totp_digits < 6 || self.totp_digits > 8 {
            return Err(crate::core::ConfigError::validation(
                "TOTP digits must be between 6 and 8",
            ));
        }
        if self.sms_otp_length < 4 || self.sms_otp_length > 8 {
            return Err(crate::core::ConfigError::validation(
                "SMS OTP length must be between 4 and 8",
            ));
        }
        if self.max_devices_per_user == 0 {
            return Err(crate::core::ConfigError::validation(
                "Max devices per user must be at least 1",
            ));
        }
        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_mfa_config_defaults() {
//         let config = MfaConfig::default();
//         assert!(config.enabled);
//         assert_eq!(config.totp_digits, 6);
//     }

//     #[test]
//     fn test_mfa_config_validation() {
//         let mut config = MfaConfig::default();
//         assert!(config.validate().is_ok());

//         config.totp_digits = 10;
//         assert!(config.validate().is_err());
//     }
// }
