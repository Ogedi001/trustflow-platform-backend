//! Rate limiting configuration
//!
//! Provides configuration types for rate limiting in the identity service.

use serde::{Deserialize, Serialize};
use time::Duration;

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Maximum login attempts per window
    pub login_attempts: u64,
    /// Login attempt window duration
    pub login_window: Duration,
    /// Maximum registrations per hour (per IP)
    pub registration_per_hour: u64,
    /// Maximum verification requests per day (per user)
    pub verification_per_day: u64,
    /// Maximum password reset requests per day (per user)
    pub password_reset_per_day: u64,
    /// Maximum OTP requests per minute (per user)
    pub otp_per_minute: u64,
    /// Maximum API requests per minute (per user)
    pub api_requests_per_minute: u64,
    /// Whether to enable IP-based rate limiting
    pub ip_based_limiting: bool,
    /// Trusted proxies for IP extraction
    pub trusted_proxies: Vec<String>,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self::from_env()
    }
}

impl RateLimitConfig {
    /// Create configuration from environment variables
    pub fn from_env() -> Self {
        Self {
            login_attempts: std::env::var("RATE_LIMIT_LOGIN_ATTEMPTS")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .unwrap_or(5),
            login_window: Duration::seconds(
                std::env::var("RATE_LIMIT_LOGIN_WINDOW")
                    .unwrap_or_else(|_| "900".to_string()) // 15 minutes
                    .parse()
                    .unwrap_or(900),
            ),
            registration_per_hour: std::env::var("RATE_LIMIT_REGISTRATION_PER_HOUR")
                .unwrap_or_else(|_| "3".to_string())
                .parse()
                .unwrap_or(3),
            verification_per_day: std::env::var("RATE_LIMIT_VERIFICATION_PER_DAY")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
            password_reset_per_day: std::env::var("RATE_LIMIT_PASSWORD_RESET_PER_DAY")
                .unwrap_or_else(|_| "3".to_string())
                .parse()
                .unwrap_or(3),
            otp_per_minute: std::env::var("RATE_LIMIT_OTP_PER_MINUTE")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .unwrap_or(5),
            api_requests_per_minute: std::env::var("RATE_LIMIT_API_REQUESTS_PER_MINUTE")
                .unwrap_or_else(|_| "60".to_string())
                .parse()
                .unwrap_or(60),
            ip_based_limiting: std::env::var("RATE_LIMIT_IP_BASED")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            trusted_proxies: std::env::var("TRUSTED_PROXIES")
                .unwrap_or_else(|_| "".to_string())
                .split(',')
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim().to_string())
                .collect(),
        }
    }

    /// Create configuration from a loader
    pub fn from_loader(loader: &crate::sources::ConfigLoader) -> crate::core::ConfigResult<Self> {
        let trusted_proxies: Vec<String> = loader
            .get_or("TRUSTED_PROXIES", "".to_string())?
            .split(',')
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().to_string())
            .collect();

        Ok(Self {
            login_attempts: loader.get_or("RATE_LIMIT_LOGIN_ATTEMPTS", 5u64)?,
            login_window: Duration::seconds(loader.get_or("RATE_LIMIT_LOGIN_WINDOW", 900i64)?),
            registration_per_hour: loader.get_or("RATE_LIMIT_REGISTRATION_PER_HOUR", 3u64)?,
            verification_per_day: loader.get_or("RATE_LIMIT_VERIFICATION_PER_DAY", 10u64)?,
            password_reset_per_day: loader.get_or("RATE_LIMIT_PASSWORD_RESET_PER_DAY", 3u64)?,
            otp_per_minute: loader.get_or("RATE_LIMIT_OTP_PER_MINUTE", 5u64)?,
            api_requests_per_minute: loader.get_or("RATE_LIMIT_API_REQUESTS_PER_MINUTE", 60u64)?,
            ip_based_limiting: loader.get_or("RATE_LIMIT_IP_BASED", true)?,
            trusted_proxies,
        })
    }

    /// Validate the configuration
    pub fn validate(&self) -> crate::core::ConfigResult<()> {
        if self.login_attempts == 0 {
            return Err(crate::core::ConfigError::validation(
                "Login attempts must be greater than 0",
            ));
        }
        if self.login_window == Duration::ZERO {
            return Err(crate::core::ConfigError::validation(
                "Login window must be greater than 0",
            ));
        }
        Ok(())
    }

    /// Get the login window in seconds
    pub fn login_window_secs(&self) -> u64 {
        self.login_window.whole_seconds()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_rate_limit_config_defaults() {
//         let config = RateLimitConfig::default();
//         assert_eq!(config.login_attempts, 5);
//         assert!(config.ip_based_limiting);
//     }

//     #[test]
//     fn test_rate_limit_config_validation() {
//         let mut config = RateLimitConfig::default();
//         assert!(config.validate().is_ok());

//         config.login_attempts = 0;
//         assert!(config.validate().is_err());
//     }
// }
