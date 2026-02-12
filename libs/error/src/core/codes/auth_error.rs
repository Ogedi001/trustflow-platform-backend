//! Authentication and authorization specific error codes
//!
//! These codes provide more specific information about auth-related failures
//! and are used within the broader Unauthorized/Forbidden categories.

use serde::{Deserialize, Serialize};

/// Authentication and authorization specific error codes
///
/// These codes provide more specific information about auth-related failures
/// and are used within the broader Unauthorized/Forbidden categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthErrorCode {
    /// Invalid username or password
    #[default]
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
