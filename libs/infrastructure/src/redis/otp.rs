//! OTP (One-Time Password) cache for Redis infrastructure
//!
//! Provides OTP storage and verification for MFA implementations.
//!
//! ## Feature Flags
//!
//! - `redis`: Enables Redis support (enabled by default with `full` feature)

#[cfg(feature = "redis")]
use async_trait::async_trait;

#[cfg(feature = "redis")]
use serde::{de::DeserializeOwned, Serialize, Deserialize};

#[cfg(feature = "redis")]
use std::time::Duration;

#[cfg(feature = "redis")]
use super::{RedisPool, RedisCache, RedisError, Cache};
use rand::{Rng, random};
use chrono::{DateTime, Utc};
#[cfg(feature = "redis")]
use crate::redis::key::RedisKey;

/// OTP data stored in Redis
#[cfg(feature = "redis")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtpData {
    pub code: String,
    pub purpose: OtpPurpose,
    pub attempts: u8,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Purpose of OTP
#[cfg(feature = "redis")]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OtpPurpose {
    /// Email verification
    EmailVerification,
    /// Phone verification
    PhoneVerification,
    /// Password reset
    PasswordReset,
    /// MFA login
    MfaLogin,
    /// Change phone number
    ChangePhone,
    /// Change email
    ChangeEmail,
    /// Two-factor setup
    MfaSetup,
}

/// OTP cache for managing one-time passwords
#[cfg(feature = "redis")]
#[derive(Clone)]
pub struct OtpCache {
    cache: RedisCache,
    max_attempts: u8,
}

#[cfg(feature = "redis")]
impl OtpCache {
    /// Create a new OTP cache
    pub fn new(pool: RedisPool, prefix: impl Into<String>, max_attempts: u8) -> Self {
        // OTP cache is just a normal RedisCache with an added ":otp" segment
        // so that other caches using the same raw prefix won't collide.
        let prefix = prefix.into();
        Self {
            cache: RedisCache::new(pool, format!("{}:otp", prefix)),
            max_attempts,
        }
    }

    /// Get prefixed key for OTP
    fn otp_key(&self, identifier: &str, purpose: OtpPurpose) -> RedisKey {
        // build using RedisKey; prefix() accessor available on RedisCache
        RedisKey::from_parts([
            self.cache.prefix(),
            purpose.as_str(),
            identifier,
        ])
    }

    /// Generate a numeric OTP
    pub fn generate_numeric(length: u8) -> String {
        let mut otp = String::new();
        for _ in 0..length {
            let digit = rand::random::<u8>() % 10;
            otp.push(std::char::from_digit(digit as u32, 10).unwrap());
        }
        otp
    }

    /// Generate an alphanumeric OTP
    pub fn generate_alphanumeric(length: u8) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
        let mut otp = String::with_capacity(length as usize);
        let mut rng = rand::thread_rng();
        for _ in 0..length {
            let idx = rng.gen::<usize>() % CHARSET.len();
            otp.push(CHARSET[idx] as char);
        }
        otp
    }

    /// Store OTP for an identifier
    pub async fn store(
        &self,
        identifier: &str,
        purpose: OtpPurpose,
        code: &str,
        ttl: Duration,
    ) -> Result<(), RedisError> {
        let otp_data = OtpData {
            code: code.to_string(),
            purpose,
            attempts: 0,
            created_at: chrono::Utc::now(),
        };

        self.cache
            .set(self.otp_key(identifier, purpose).as_str(), &otp_data, ttl)
            .await
    }

    /// Verify OTP for an identifier
    pub async fn verify(
        &self,
        identifier: &str,
        purpose: OtpPurpose,
        code: &str,
    ) -> Result<OtpVerifyResult, RedisError> {
        let key = self.otp_key(identifier, purpose);
        
        let otp_data: Option<OtpData> = self.cache.get(key.as_str()).await?;
        
        match otp_data {
            Some(data) => {
                // Check if code matches
                if data.code == code {
                    // Check if attempts exceeded
                    if data.attempts >= self.max_attempts {
                        // Delete the OTP as it's exhausted
                        self.cache.delete(&key).await?;
                        return Ok(OtpVerifyResult::MaxAttemptsExceeded);
                    }
                    
                    // Delete the OTP after successful verification
                    self.cache.delete(&key).await?;
                    Ok(OtpVerifyResult::Valid)
                } else {
                    // Increment attempts
                    let mut updated_data = data;
                    updated_data.attempts += 1;
                    self.cache
                        .set(key.as_str(), &updated_data, Duration::from_secs(300))
                        .await?;
                    
                    if updated_data.attempts >= self.max_attempts {
                        self.cache.delete(&key).await?;
                        return Ok(OtpVerifyResult::MaxAttemptsExceeded);
                    }
                    
                    Ok(OtpVerifyResult::Invalid {
                        attempts_remaining: self.max_attempts - updated_data.attempts,
                    })
                }
            }
            None => Ok(OtpVerifyResult::NotFound),
        }
    }

    /// Check if OTP exists for an identifier
    pub async fn exists(&self, identifier: &str, purpose: OtpPurpose) -> Result<bool, RedisError> {
        self.cache.exists(&self.otp_key(identifier, purpose)).await
    }

    /// Delete OTP for an identifier
    pub async fn delete(&self, identifier: &str, purpose: OtpPurpose) -> Result<(), RedisError> {
        self.cache.delete(&self.otp_key(identifier, purpose)).await
    }

    /// Get remaining attempts for an identifier
    pub async fn remaining_attempts(&self, identifier: &str, purpose: OtpPurpose) -> Result<u8, RedisError> {
        let otp_data: Option<OtpData> = self.cache.get(&self.otp_key(identifier, purpose)).await?;
        
        match otp_data {
            Some(data) => Ok(self.max_attempts - data.attempts),
            None => Ok(self.max_attempts),
        }
    }
}

/// Result of OTP verification
#[cfg(feature = "redis")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OtpVerifyResult {
    /// OTP is valid and was consumed
    Valid,
    /// OTP is invalid
    Invalid { attempts_remaining: u8 },
    /// Maximum attempts exceeded, OTP was consumed
    MaxAttemptsExceeded,
    /// OTP not found (may have expired or already used)
    NotFound,
}

#[cfg(feature = "redis")]
impl OtpPurpose {
    /// Get string representation for Redis key
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EmailVerification => "email_verification",
            Self::PhoneVerification => "phone_verification",
            Self::PasswordReset => "password_reset",
            Self::MfaLogin => "mfa_login",
            Self::ChangePhone => "change_phone",
            Self::ChangeEmail => "change_email",
            Self::MfaSetup => "mfa_setup",
        }
    }
}

