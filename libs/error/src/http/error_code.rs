//! HTTP error codes for API responses
//!
//! Provides standardized error codes that are used across all services
//! for consistent API error responses.

use serde::{Deserialize, Serialize};

/// Standard error codes for HTTP API responses
///
/// These codes are designed to be:
/// - Self-explanatory and human-readable
/// - Consistent across all services
/// - Compatible with HTTP status codes
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    /// 400 Bad Request - The request was invalid or cannot be served
    BadRequest,

    /// 401 Unauthorized - Authentication is required
    Unauthorized,

    /// 403 Forbidden - The server understood the request but refuses to authorize it
    Forbidden,

    /// 404 Not Found - The requested resource could not be found
    NotFound,

    /// 409 Conflict - The request conflicts with the current state of the resource
    Conflict,

    /// 422 Unprocessable Entity - The request was well-formed but contains invalid data
    ValidationError,

    /// 429 Too Many Requests - Rate limiting applied
    RateLimited,

    /// 500 Internal Server Error - An unexpected server error occurred
    InternalError,

    /// 503 Service Unavailable - The server is temporarily unavailable
    ServiceUnavailable,

    /// 502 Bad Gateway - Invalid response from upstream server
    BadGateway,
}

/// Authentication and authorization specific error codes
///
/// These codes provide more specific information about auth-related failures
/// and are used within the broader Unauthorized/Forbidden categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthErrorCode {
    /// Invalid username or password
    InvalidCredentials,

    /// Account has been locked due to security reasons
    AccountLocked,

    /// Account has been suspended by an administrator
    AccountSuspended,

    /// Account has been permanently deleted
    AccountDeleted,

    /// Multi-factor authentication is required to proceed
    MfaRequired,

    /// The provided MFA token/code is invalid
    MfaInvalid,

    /// The MFA token has expired and a new one is needed
    MfaExpired,

    /// The authentication token has expired
    TokenExpired,

    /// The authentication token is invalid
    TokenInvalid,

    /// The authentication token has been revoked
    TokenRevoked,

    /// The session has expired
    SessionExpired,

    /// The session is invalid
    SessionInvalid,

    /// The password has expired and needs to be changed
    PasswordExpired,

    /// The password does not meet strength requirements
    PasswordWeak,

    /// The provided passwords do not match
    PasswordMismatch,

    /// Rate limiting applied to authentication attempts
    RateLimited,

    /// IP address has been blocked
    IpBlocked,

    /// Device has been blocked
    DeviceBlocked,

    /// Token is missing from the request
    TokenMissing,

    /// Insufficient permissions to perform the action
    InsufficientPermissions,

    /// Account is not active
    AccountInactive,

    /// Email has not been verified
    EmailNotVerified,

    /// Phone has not been verified
    PhoneNotVerified,

    /// Social login is required
    SocialLoginRequired,

    /// The invite code is invalid or expired
    InvalidInviteCode,
}

impl ErrorCode {
    /// Get the default HTTP status code for this error code
    pub fn status_code(&self) -> u16 {
        match self {
            Self::BadRequest => 400,
            Self::Unauthorized => 401,
            Self::Forbidden => 403,
            Self::NotFound => 404,
            Self::Conflict => 409,
            Self::ValidationError => 422,
            Self::RateLimited => 429,
            Self::InternalError => 500,
            Self::ServiceUnavailable => 503,
            Self::BadGateway => 502,
        }
    }
}

impl AuthErrorCode {
    /// Get the parent HTTP error code for this auth error code
    pub fn parent_error_code(&self) -> ErrorCode {
        match self {
            Self::InsufficientPermissions => ErrorCode::Forbidden,
            Self::AccountLocked
            | Self::AccountSuspended
            | Self::AccountDeleted
            | Self::AccountInactive
            | Self::TokenExpired
            | Self::TokenRevoked
            | Self::SessionExpired
            | Self::SessionInvalid
            | Self::PasswordExpired
            | Self::TokenMissing => ErrorCode::Unauthorized,
            Self::InvalidCredentials
            | Self::MfaRequired
            | Self::MfaInvalid
            | Self::MfaExpired
            | Self::PasswordWeak
            | Self::PasswordMismatch
            | Self::RateLimited
            | Self::IpBlocked
            | Self::DeviceBlocked
            | Self::EmailNotVerified
            | Self::PhoneNotVerified
            | Self::SocialLoginRequired
            | Self::InvalidInviteCode => ErrorCode::Unauthorized,
        }
    }
}
