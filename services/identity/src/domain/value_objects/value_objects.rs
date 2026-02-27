//! Value objects for Identity Service
//!
//! Simple value objects for validation and business rules.

use crate::domain::enums::*;
use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Password value object with validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Password {
    value: String,
    hash: Option<String>,
}

impl Password {
    /// Create a new password and hash it
    pub fn new(value: String) -> AppResult<Self> {
        Self::validate(&value)?;
        Ok(Self { value, hash: None })
    }

    /// Create from already hashed password
    pub fn from_hash(value: String, hash: String) -> Self {
        Self {
            value,
            hash: Some(hash),
        }
    }

    /// Validate password strength
    fn validate(value: &str) -> AppResult<()> {
        if value.len() < 8 {
            return Err(AppError::ValidationError(
                "Password must be at least 8 characters".to_string(),
            ));
        }

        if !value.chars().any(|c| c.is_uppercase()) {
            return Err(AppError::ValidationError(
                "Password must contain at least one uppercase letter".to_string(),
            ));
        }

        if !value.chars().any(|c| c.is_lowercase()) {
            return Err(AppError::ValidationError(
                "Password must contain at least one lowercase letter".to_string(),
            ));
        }

        if !value.chars().any(|c| c.is_ascii_digit()) {
            return Err(AppError::ValidationError(
                "Password must contain at least one digit".to_string(),
            ));
        }

        if !value.chars().any(|c| !c.is_alphanumeric()) {
            return Err(AppError::ValidationError(
                "Password must contain at least one special character".to_string(),
            ));
        }

        Ok(())
    }

    /// Get password value (plain text - use carefully)
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Get hash
    pub fn hash(&self) -> Option<&str> {
        self.hash.as_deref()
    }

    /// Set hash
    pub fn set_hash(&mut self, hash: String) {
        self.hash = Some(hash);
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Password(**hidden**)")
    }
}

/// OTP value object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Otp {
    value: String,
    purpose: OtpPurpose,
    expires_at: time::OffsetDateTime,
}

impl Otp {
    /// Create a new OTP
    pub fn new(value: String, purpose: OtpPurpose, duration_minutes: i64) -> Self {
        Self {
            value,
            purpose,
            expires_at: time::OffsetDateTime::now_utc() + time::Duration::minutes(duration_minutes),
        }
    }

    /// Create a numeric OTP of specified length
    pub fn generate_numeric(length: u32, purpose: OtpPurpose, duration_minutes: i64) -> Self {
        let value: String = std::iter::repeat(())
            .map(|()| fastrand::digit())
            .take(length as usize)
            .collect();
        Self::new(value, purpose, duration_minutes)
    }

    /// Check if OTP is expired
    pub fn is_expired(&self) -> bool {
        self.expires_at < time::OffsetDateTime::now_utc()
    }

    /// Get the OTP value
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Get the purpose
    pub fn purpose(&self) -> OtpPurpose {
        self.purpose
    }
}

/// OTP purpose enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OtpPurpose {
    EmailVerification,
    PhoneVerification,
    PasswordReset,
    LoginMfa,
    TransactionVerification,
}

/// Invite code value object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct InviteCode {
    code: String,
    role: UserRole,
    max_uses: u32,
    used_count: u32,
    expires_at: Option<time::OffsetDateTime>,
}

impl InviteCode {
    /// Create a new invite code
    pub fn new(
        code: String,
        role: UserRole,
        max_uses: u32,
        expires_at: Option<time::OffsetDateTime>,
    ) -> Self {
        Self {
            code,
            role,
            max_uses,
            used_count: 0,
            expires_at,
        }
    }

    /// Generate a random invite code
    pub fn generate(
        role: UserRole,
        max_uses: u32,
        expires_at: Option<time::OffsetDateTime>,
    ) -> Self {
        let code: String = std::iter::repeat(())
            .map(|()| fastrand::char())
            .take(8)
            .collect();
        Self::new(code.to_uppercase(), role, max_uses, expires_at)
    }

    /// Check if code is valid
    pub fn is_valid(&self) -> bool {
        if self.used_count >= self.max_uses {
            return false;
        }
        if let Some(expires) = self.expires_at {
            if expires < time::OffsetDateTime::now_utc() {
                return false;
            }
        }
        true
    }

    /// Mark code as used
    pub fn mark_used(&mut self) {
        self.used_count += 1;
    }

    /// Get the code value
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Get assigned role
    pub fn role(&self) -> UserRole {
        self.role
    }
}

/// Device fingerprint value object
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeviceFingerprint {
    fingerprint: String,
    user_agent: String,
    platform: String,
    browser: String,
}

impl DeviceFingerprint {
    /// Create a new device fingerprint
    pub fn new(fingerprint: String, user_agent: String, platform: String, browser: String) -> Self {
        Self {
            fingerprint,
            user_agent,
            platform,
            browser,
        }
    }

    /// Get fingerprint
    pub fn fingerprint(&self) -> &str {
        &self.fingerprint
    }
}

/// Trust score value object (0-1000)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TrustScore(pub i32);

impl TrustScore {
    /// Create a new trust score
    pub fn new(value: i32) -> Self {
        Self(value.clamp(0, 1000))
    }

    /// Default trust score for new users
    pub fn default_for(level: VerificationLevel) -> Self {
        Self(level.trust_score_boost())
    }

    /// Boost trust score
    pub fn boost(&mut self, amount: i32) {
        self.0 = (self.0 + amount).clamp(0, 1000);
    }

    /// Deduct trust score
    pub fn deduct(&mut self, amount: i32) {
        self.0 = (self.0 - amount).clamp(0, 1000);
    }

    /// Get the value
    pub fn value(&self) -> i32 {
        self.0
    }

    /// Check if trusted
    pub fn is_trusted(&self) -> bool {
        self.0 >= 500
    }

    /// Check if highly trusted
    pub fn is_highly_trusted(&self) -> bool {
        self.0 >= 800
    }
}

impl fmt::Display for TrustScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TrustScore({})", self.0)
    }
}
