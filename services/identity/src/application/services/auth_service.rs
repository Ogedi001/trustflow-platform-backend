//! Authentication Service for Identity Service
//!
//! Handles user registration, login, logout, MFA, and token management.

use crate::{
    application::config::Config,
    domain::{entities::*, enums::*},
    infrastructure::Infrastructure,
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use base32::Alphabet;
use common::{EmailAddress, PasswordHash as CommonPasswordHash, PhoneNumber, UserId};
use error::{AppError, http::AuthErrorCode};
use rand::RngCore;
use rand::rngs::OsRng;
use thiserror::Error;
use time::Duration;

/// Authentication service errors
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Account locked")]
    AccountLocked,

    #[error("Account suspended: {0}")]
    AccountSuspended(String),

    #[error("Account deleted")]
    AccountDeleted,

    #[error("MFA required")]
    MfaRequired,

    #[error("MFA token expired")]
    MfaTokenExpired,

    #[error("Invalid MFA token")]
    InvalidMfaToken,

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Email already exists")]
    EmailAlreadyExists,

    #[error("Phone number already exists")]
    PhoneAlreadyExists,

    #[error("Invalid email format")]
    InvalidEmailFormat,

    #[error("Invalid phone format")]
    InvalidPhoneFormat,

    #[error("Password too weak")]
    WeakPassword,

    #[error("Invalid invite code")]
    InvalidInviteCode,
}

/// Authentication result
#[derive(Debug)]
pub struct AuthResult {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub token_type: String,
    pub user: UserResult,
}

/// User result for auth response
#[derive(Debug)]
pub struct UserResult {
    pub id: UserId,
    pub email: String,
    pub phone: String,
    pub role: String,
    pub verification_level: u8,
}

/// Authentication service
#[derive(Clone)]
pub struct AuthService {
    infrastructure: Infrastructure,
    config: Config,
    jwt_secret: String,
}

impl AuthService {
    /// Create new authentication service
    pub fn new(infrastructure: Infrastructure, config: Config) -> Self {
        Self {
            infrastructure,
            config: config.clone(),
            jwt_secret: config.jwt.secret.clone(),
        }
    }

    /// Register a new user
    pub async fn register(
        &self,
        email: &str,
        phone: &str,
        password: &str,
        role: UserRole,
        invite_code: Option<&str>,
    ) -> Result<UserId, AuthError> {
        // Validate email
        let email = EmailAddress(email.to_string());
        if !email.is_valid() {
            return Err(AuthError::InvalidEmailFormat);
        }

        // Validate phone
        let phone = PhoneNumber(phone.to_string());
        if !phone.is_valid() {
            return Err(AuthError::InvalidPhoneFormat);
        }

        // Validate password
        self.validate_password(password)?;

        // Check for existing user
        // This would query the database
        // For now, return a placeholder

        // Hash password
        let password_hash = self.hash_password(password)?;

        // Create user
        let user = User::new_pending(email, phone, CommonPasswordHash(password_hash), role);

        // Save user to database
        // This would call the repository

        Ok(user.id)
    }

    /// Authenticate user
    pub async fn login(
        &self,
        identifier: &str,
        password: &str,
        device_id: &str,
        user_agent: &str,
        ip_address: &str,
    ) -> Result<AuthResult, AuthError> {
        // Find user by email or phone
        // This would query the database

        // Check rate limiting
        // This would use Redis rate limiter

        // Verify password
        // This would verify against stored hash

        // Check account status
        // This would check if account is active

        // Check if MFA is required
        // If required, return MfaRequired error

        // Update last login
        // This would update the user record

        // Generate tokens
        let access_token = self.generate_access_token("user_id", "email", "BUYER", device_id)?;
        let refresh_token = self.generate_refresh_token("user_id", "email", "BUYER", device_id)?;

        Ok(AuthResult {
            access_token,
            refresh_token,
            expires_in: 3600,
            token_type: "Bearer".to_string(),
            user: UserResult {
                id: UserId::new(),
                email: identifier.to_string(),
                phone: "+2340000000000".to_string(),
                role: "BUYER".to_string(),
                verification_level: 0,
            },
        })
    }

    /// Refresh access token
    pub async fn refresh_access_token(&self, refresh_token: &str) -> Result<AuthResult, AuthError> {
        // Validate refresh token
        // This would verify the token

        // Generate new access token
        let access_token = self.generate_access_token("user_id", "email", "BUYER", "device_id")?;
        let new_refresh_token =
            self.generate_refresh_token("user_id", "email", "BUYER", "device_id")?;

        Ok(AuthResult {
            access_token,
            refresh_token: new_refresh_token,
            expires_in: 3600,
            token_type: "Bearer".to_string(),
            user: UserResult {
                id: UserId::new(),
                email: "placeholder@example.com".to_string(),
                phone: "+2340000000000".to_string(),
                role: "BUYER".to_string(),
                verification_level: 0,
            },
        })
    }

    /// Logout user
    pub async fn logout(&self, user_id: &UserId, session_id: &str) -> Result<(), AuthError> {
        // Revoke session
        // This would update the session in Redis/Database

        Ok(())
    }

    /// Logout from all sessions
    pub async fn logout_all_sessions(&self, user_id: &UserId) -> Result<(), AuthError> {
        // Revoke all sessions for user
        // This would delete all sessions from Redis/Database

        Ok(())
    }

    /// Change password
    pub async fn change_password(
        &self,
        user_id: &UserId,
        old_password: &str,
        new_password: &str,
    ) -> Result<(), AuthError> {
        // Validate new password
        self.validate_password(new_password)?;

        // Verify old password
        // This would check the current password

        // Hash new password
        let new_hash = self.hash_password(new_password)?;

        // Update password in database
        // This would update the user record

        Ok(())
    }

    /// Enable MFA for user
    pub async fn enable_mfa(
        &self,
        user_id: &UserId,
        method: MfaMethod,
    ) -> Result<String, AuthError> {
        match method {
            MfaMethod::Totp => {
                // Generate TOTP secret
                let mut secret_bytes = [0u8; 20];
                OsRng.fill_bytes(&mut secret_bytes);
                let secret = base32::encode(Alphabet::RFC4648 { padding: false }, &secret_bytes);

                // Store secret for user
                // This would update the user record

                Ok(secret)
            }
            MfaMethod::Sms | MfaMethod::Email => {
                // Generate OTP
                let otp = self.generate_otp(6);

                // Store OTP with TTL
                // This would store in Redis

                Ok(otp)
            }
            _ => Err(AuthError::InvalidMfaToken),
        }
    }

    /// Verify MFA token
    pub async fn verify_mfa(&self, user_id: &UserId, token: &str) -> Result<bool, AuthError> {
        // Verify TOTP or OTP
        // This would check the stored secret/OTP

        Ok(true)
    }

    /// Disable MFA
    pub async fn disable_mfa(&self, user_id: &UserId, password: &str) -> Result<(), AuthError> {
        // Verify password
        // This would check the current password

        // Remove MFA secret
        // This would update the user record

        Ok(())
    }

    /// Validate password strength
    fn validate_password(&self, password: &str) -> Result<(), AuthError> {
        let config = &self.config.password;

        if password.len() < config.min_length as usize {
            return Err(AuthError::WeakPassword);
        }

        if config.require_uppercase && !password.chars().any(|c| c.is_uppercase()) {
            return Err(AuthError::WeakPassword);
        }

        if config.require_lowercase && !password.chars().any(|c| c.is_lowercase()) {
            return Err(AuthError::WeakPassword);
        }

        if config.require_digit && !password.chars().any(|c| c.is_ascii_digit()) {
            return Err(AuthError::WeakPassword);
        }

        if config.require_special && !password.chars().any(|c| !c.is_alphanumeric()) {
            return Err(AuthError::WeakPassword);
        }

        Ok(())
    }

    /// Hash password using Argon2id
    fn hash_password(&self, password: &str) -> Result<String, AuthError> {
        let argon2 = Argon2::default();
        let salt = self.generate_salt();

        let password_bytes = password.as_bytes();
        let mut hash = [0u8; 32];

        argon2
            .hash_password_into(password_bytes, &salt, &mut hash)
            .map_err(|_| AuthError::InvalidCredentials)?;

        Ok(format!(
            "$argon2id$v=19$m=19456,t=2,p=1${}${}",
            base64::encode(salt),
            base64::encode(hash)
        ))
    }

    /// Verify password
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, AuthError> {
        let parsed_hash = PasswordHash::new(hash).map_err(|_| AuthError::InvalidCredentials)?;

        let argon2 = Argon2::default();

        Ok(argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    /// Generate salt
    fn generate_salt(&self) -> [u8; 32] {
        let mut salt = [0u8; 32];
        OsRng.fill_bytes(&mut salt);
        salt
    }

    /// Generate OTP
    fn generate_otp(&self, length: u8) -> String {
        let mut otp = String::new();
        for _ in 0..length {
            otp.push(std::char::from_digit(OsRng.next_u32() % 10, 10).unwrap());
        }
        otp
    }

    /// Generate JWT access token
    fn generate_access_token(
        &self,
        user_id: &str,
        email: &str,
        role: &str,
        device_id: &str,
    ) -> Result<String, AuthError> {
        let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
        let exp = jsonwebtoken::get_current_timestamp() + 3600;

        let payload = jsonwebtoken::Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            role: role.to_string(),
            session_id: "session_id".to_string(),
            device_id: device_id.to_string(),
            exp,
            iat: jsonwebtoken::get_current_timestamp(),
            iss: "trustflow-identity".to_string(),
            aud: "trustflow".to_string(),
        };

        jsonwebtoken::encode(
            &header,
            &payload,
            &jsonwebtoken::EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| AuthError::InvalidCredentials)
    }

    /// Generate JWT refresh token
    fn generate_refresh_token(
        &self,
        user_id: &str,
        email: &str,
        role: &str,
        device_id: &str,
    ) -> Result<String, AuthError> {
        let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256);
        let exp = jsonwebtoken::get_current_timestamp() + 604800; // 7 days

        let payload = jsonwebtoken::Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            role: role.to_string(),
            session_id: "session_id".to_string(),
            device_id: device_id.to_string(),
            exp,
            iat: jsonwebtoken::get_current_timestamp(),
            iss: "trustflow-identity".to_string(),
            aud: "trustflow".to_string(),
        };

        jsonwebtoken::encode(
            &header,
            &payload,
            &jsonwebtoken::EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| AuthError::InvalidCredentials)
    }

    /// Get current user ID from context
    pub async fn get_current_user_id(&self) -> Option<UserId> {
        // This would extract the user ID from the request context
        None
    }
}

impl From<AuthError> for AppError {
    fn from(e: AuthError) -> Self {
        match e {
            AuthError::InvalidCredentials => {
                AppError::auth("Invalid credentials", AuthErrorCode::InvalidCredentials)
            }
            AuthError::AccountLocked => {
                AppError::auth("Account locked", AuthErrorCode::AccountLocked)
            }
            AuthError::AccountSuspended(reason) => AppError::auth(
                &format!("Account suspended: {}", reason),
                AuthErrorCode::AccountSuspended,
            ),
            AuthError::AccountDeleted => {
                AppError::auth("Account deleted", AuthErrorCode::AccountDeleted)
            }
            AuthError::MfaRequired => AppError::auth("MFA required", AuthErrorCode::MfaRequired),
            AuthError::MfaTokenExpired => {
                AppError::auth("MFA token expired", AuthErrorCode::MfaExpired)
            }
            AuthError::InvalidMfaToken => {
                AppError::auth("Invalid MFA token", AuthErrorCode::MfaInvalid)
            }
            AuthError::RateLimitExceeded => AppError::rate_limited("Rate limit exceeded", 0),
            AuthError::EmailAlreadyExists => AppError::conflict("Email already exists"),
            AuthError::PhoneAlreadyExists => AppError::conflict("Phone number already exists"),
            AuthError::InvalidEmailFormat => AppError::bad_request("Invalid email format"),
            AuthError::InvalidPhoneFormat => AppError::bad_request("Invalid phone format"),
            AuthError::WeakPassword => AppError::bad_request("Password too weak"),
            AuthError::InvalidInviteCode => AppError::bad_request("Invalid invite code"),
        }
    }
}
