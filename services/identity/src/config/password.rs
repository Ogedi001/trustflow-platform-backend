//! Password configuration
//!
//! Provides configuration types for password policies in the identity service.

use serde::{Deserialize, Serialize};

/// Password configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordConfig {
    /// Minimum password length
    pub min_length: u8,
    /// Whether to require uppercase letter
    pub require_uppercase: bool,
    /// Whether to require lowercase letter
    pub require_lowercase: bool,
    /// Whether to require digit
    pub require_digit: bool,
    /// Whether to require special character
    pub require_special: bool,
    /// Maximum password age in days (0 = no expiration)
    pub max_age_days: u32,
    /// Number of passwords to keep in history
    pub history_count: u8,
    /// Whether to enable password strength meter
    pub strength_meter_enabled: bool,
    /// Maximum failed login attempts before lockout
    pub max_failed_attempts: u32,
    /// Lockout duration in minutes
    pub lockout_duration_minutes: u32,
    /// Whether to require password change on first login
    pub require_change_on_first_login: bool,
    /// Special characters that are allowed
    pub allowed_special_chars: String,
}

impl Default for PasswordConfig {
    fn default() -> Self {
        Self::from_env()
    }
}

impl PasswordConfig {
    /// Create configuration from environment variables
    pub fn from_env() -> Self {
        Self {
            min_length: std::env::var("PASSWORD_MIN_LENGTH")
                .unwrap_or_else(|_| "8".to_string())
                .parse()
                .unwrap_or(8),
            require_uppercase: std::env::var("PASSWORD_REQUIRE_UPPERCASE")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            require_lowercase: std::env::var("PASSWORD_REQUIRE_LOWERCASE")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            require_digit: std::env::var("PASSWORD_REQUIRE_DIGIT")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            require_special: std::env::var("PASSWORD_REQUIRE_SPECIAL")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            max_age_days: std::env::var("PASSWORD_MAX_AGE_DAYS")
                .unwrap_or_else(|_| "90".to_string())
                .parse()
                .unwrap_or(90),
            history_count: std::env::var("PASSWORD_HISTORY_COUNT")
                .unwrap_or_else(|_| "12".to_string())
                .parse()
                .unwrap_or(12),
            strength_meter_enabled: std::env::var("PASSWORD_STRENGTH_METER_ENABLED")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            max_failed_attempts: std::env::var("PASSWORD_MAX_FAILED_ATTEMPTS")
                .unwrap_or_else(|_| "5".to_string())
                .parse()
                .unwrap_or(5),
            lockout_duration_minutes: std::env::var("PASSWORD_LOCKOUT_DURATION_MINUTES")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
            require_change_on_first_login: std::env::var("PASSWORD_REQUIRE_CHANGE_ON_FIRST_LOGIN")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            allowed_special_chars: std::env::var("PASSWORD_ALLOWED_SPECIAL_CHARS")
                .unwrap_or_else(|_| "!@#$%^&*()_+-=[]{}|;:,.<>?".to_string()),
        }
    }

    /// Create configuration from a loader
    pub fn from_loader(loader: &crate::sources::ConfigLoader) -> crate::core::ConfigResult<Self> {
        Ok(Self {
            min_length: loader.get_or("PASSWORD_MIN_LENGTH", 8u8)?,
            require_uppercase: loader.get_or("PASSWORD_REQUIRE_UPPERCASE", true)?,
            require_lowercase: loader.get_or("PASSWORD_REQUIRE_LOWERCASE", true)?,
            require_digit: loader.get_or("PASSWORD_REQUIRE_DIGIT", true)?,
            require_special: loader.get_or("PASSWORD_REQUIRE_SPECIAL", true)?,
            max_age_days: loader.get_or("PASSWORD_MAX_AGE_DAYS", 90u32)?,
            history_count: loader.get_or("PASSWORD_HISTORY_COUNT", 12u8)?,
            strength_meter_enabled: loader.get_or("PASSWORD_STRENGTH_METER_ENABLED", true)?,
            max_failed_attempts: loader.get_or("PASSWORD_MAX_FAILED_ATTEMPTS", 5u32)?,
            lockout_duration_minutes: loader.get_or("PASSWORD_LOCKOUT_DURATION_MINUTES", 30u32)?,
            require_change_on_first_login: loader
                .get_or("PASSWORD_REQUIRE_CHANGE_ON_FIRST_LOGIN", false)?,
            allowed_special_chars: loader.get_or(
                "PASSWORD_ALLOWED_SPECIAL_CHARS",
                "!@#$%^&*()_+-=[]{}|;:,.<>?".to_string(),
            )?,
        })
    }

    /// Validate the configuration
    pub fn validate(&self) -> crate::core::ConfigResult<()> {
        if self.min_length < 8 {
            return Err(crate::core::ConfigError::validation(
                "Minimum password length must be at least 8",
            ));
        }
        if self.min_length > 128 {
            return Err(crate::core::ConfigError::validation(
                "Minimum password length must not exceed 128",
            ));
        }
        if self.history_count > 24 {
            return Err(crate::core::ConfigError::validation(
                "Password history count must not exceed 24",
            ));
        }
        if !self.require_uppercase
            && !self.require_lowercase
            && !self.require_digit
            && !self.require_special
        {
            return Err(crate::core::ConfigError::validation(
                "At least one password character requirement must be enabled",
            ));
        }
        Ok(())
    }

    /// Get the password requirements as a regex pattern string
    pub fn requirements_pattern(&self) -> String {
        let mut pattern = String::from("^");

        // Length
        pattern.push_str(&format!(".{{{},}}", self.min_length));

        // Character classes
        if self.require_uppercase {
            pattern.push_str("(?=.*[A-Z])");
        }
        if self.require_lowercase {
            pattern.push_str("(?=.*[a-z])");
        }
        if self.require_digit {
            pattern.push_str("(?=.*\\d)");
        }
        if self.require_special {
            pattern.push_str(&format!(
                "(?=.*[{}])",
                regex::escape(&self.allowed_special_chars)
            ));
        }

        pattern.push('$');
        pattern
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_config_defaults() {
        let config = PasswordConfig::default();
        assert_eq!(config.min_length, 8);
        assert!(config.require_uppercase);
    }

    #[test]
    fn test_password_config_validation() {
        let mut config = PasswordConfig::default();
        assert!(config.validate().is_ok());

        config.min_length = 4;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_requirements_pattern() {
        let config = PasswordConfig::default();
        let pattern = config.requirements_pattern();
        assert!(!pattern.is_empty());
    }
}
