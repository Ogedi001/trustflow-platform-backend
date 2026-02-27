//! Domain enums for Identity Service
//!
//! Core enums defining user roles, statuses, verification levels, etc.

use serde::{Deserialize, Serialize};
// use sqlx::Type;
use strum_macros::{Display, EnumString};
use thiserror::Error;

/// User status enum - defines current account state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
//#[sqlx(type_name = "user_status", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserStatus {
    /// Account created, awaiting email/phone verification
    Pending = 0,
    /// Account active and in good standing
    Active = 1,
    /// Account temporarily disabled
    Suspended = 2,
    /// Account permanently deleted (soft delete)
    Deleted = 3,
    /// Account locked due to security reasons
    Locked = 4,
    /// Account awaiting approval (e.g., business accounts)
    AwaitingApproval = 5,
}

impl Default for UserStatus {
    fn default() -> Self {
        Self::Pending
    }
}

impl UserStatus {
    /// Check if user can authenticate
    pub fn can_authenticate(&self) -> bool {
        matches!(self, Self::Active)
    }

    /// Check if account is in terminal state
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Deleted)
    }

    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Pending => "Pending Verification",
            Self::Active => "Active",
            Self::Suspended => "Suspended",
            Self::Deleted => "Deleted",
            Self::Locked => "Locked",
            Self::AwaitingApproval => "Awaiting Approval",
        }
    }
}

/// Verification level enum - tiered identity verification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
//#[sqlx(type_name = "verification_level", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VerificationLevel {
    /// Level 0: Email/Phone only (basic)
    Level0 = 0,
    /// Level 1: Basic KYC (Name, Phone, Email verified)
    Level1 = 1,
    /// Level 2: Document Verification (Government ID)
    Level2 = 2,
    /// Level 3: Business Verification (CAC, Tax ID)
    Level3 = 3,
    /// Level 4: Enhanced Due Diligence (Face verification, biometrics)
    Level4 = 4,
}

impl Default for VerificationLevel {
    fn default() -> Self {
        Self::Level0
    }
}

impl VerificationLevel {
    /// Get next level (if any)
    pub fn next_level(&self) -> Option<Self> {
        match self {
            Self::Level0 => Some(Self::Level1),
            Self::Level1 => Some(Self::Level2),
            Self::Level2 => Some(Self::Level3),
            Self::Level3 => Some(Self::Level4),
            Self::Level4 => None,
        }
    }

    /// Get required profile completeness for this level
    pub fn requires_personal_info(&self) -> bool {
        matches!(
            self,
            Self::Level1 | Self::Level2 | Self::Level3 | Self::Level4
        )
    }

    /// Get required document type for this level
    pub fn requires_document(&self) -> bool {
        matches!(self, Self::Level2 | Self::Level3 | Self::Level4)
    }

    /// Get required business info for this level
    pub fn requires_business_info(&self) -> bool {
        matches!(self, Self::Level3 | Self::Level4)
    }

    /// Get trust score boost for this level
    pub fn trust_score_boost(&self) -> i32 {
        match self {
            Self::Level0 => 100,
            Self::Level1 => 200,
            Self::Level2 => 400,
            Self::Level3 => 600,
            Self::Level4 => 800,
        }
    }

    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Level0 => "Basic (Email/Phone)",
            Self::Level1 => "Identity Verified",
            Self::Level2 => "Document Verified",
            Self::Level3 => "Business Verified",
            Self::Level4 => "Fully Verified",
        }
    }
}

/// Verification status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
//#[sqlx(type_name = "verification_status", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VerificationStatus {
    /// Verification in progress
    Pending = 0,
    /// Verification approved
    Approved = 1,
    /// Verification rejected
    Rejected = 2,
    /// Verification expired
    Expired = 3,
    /// Needs manual review
    ManualReview = 4,
    /// Verification cancelled by user
    Cancelled = 5,
}

impl Default for VerificationStatus {
    fn default() -> Self {
        Self::Pending
    }
}

impl VerificationStatus {
    /// Check if verification is successful
    pub fn is_successful(&self) -> bool {
        matches!(self, Self::Approved)
    }

    /// Check if verification is in progress
    pub fn is_pending(&self) -> bool {
        matches!(self, Self::Pending | Self::ManualReview)
    }

    /// Check if verification needs action
    pub fn needs_action(&self) -> bool {
        matches!(self, Self::Rejected | Self::Expired | Self::Cancelled)
    }
}

/// Verification method enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
//#[sqlx(type_name = "verification_method", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VerificationMethod {
    /// Email verification link
    Email = 1,
    /// SMS OTP verification
    Phone = 2,
    /// Government ID document verification
    Document = 3,
    /// Manual verification by admin
    Manual = 4,
    /// Biometric verification
    Biometric = 5,
    /// Social media account linking
    Social = 6,
    /// Third-party KYC provider
    Provider = 7,
    /// Bank account verification
    Bank = 8,
}

impl Default for VerificationMethod {
    fn default() -> Self {
        Self::Email
    }
}

/// Document type enum for Nigerian documents
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
//#[sqlx(type_name = "document_type", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DocumentType {
    /// National Identification Number (NIN)
    Nin = 1,
    /// Driver's License
    DriversLicense = 2,
    /// International Passport
    InternationalPassport = 3,
    /// CAC Registration Certificate
    CacCertificate = 4,
    /// Tax Identification Number (TIN)
    Tin = 5,
    /// Voters Card
    VotersCard = 6,
    /// Permanent Voters Card (PVC)
    Pvc = 7,
    /// Other government issued ID
    Other = 8,
}

impl Default for DocumentType {
    fn default() -> Self {
        Self::Nin
    }
}

impl DocumentType {
    /// Get document name
    pub fn name(&self) -> &'static str {
        match self {
            Self::Nin => "National Identification Number (NIN)",
            Self::DriversLicense => "Driver's License",
            Self::InternationalPassport => "International Passport",
            Self::CacCertificate => "CAC Registration Certificate",
            Self::Tin => "Tax Identification Number (TIN)",
            Self::VotersCard => "Voter's Card",
            Self::Pvc => "Permanent Voter's Card (PVC)",
            Self::Other => "Other Government ID",
        }
    }

    /// Check if document is personal ID
    pub fn is_personal_id(&self) -> bool {
        matches!(
            self,
            Self::Nin
                | Self::DriversLicense
                | Self::InternationalPassport
                | Self::VotersCard
                | Self::Pvc
        )
    }

    /// Check if document is business ID
    pub fn is_business_id(&self) -> bool {
        matches!(self, Self::CacCertificate | Self::Tin)
    }
}

/// MFA method enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
//#[sqlx(type_name = "mfa_method", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MfaMethod {
    /// Time-based OTP (Google Authenticator, etc.)
    Totp = 1,
    /// SMS OTP
    Sms = 2,
    /// Email OTP
    Email = 3,
    /// Push notification
    Push = 4,
    /// WebAuthn / FIDO2
    Webauthn = 5,
}

impl Default for MfaMethod {
    fn default() -> Self {
        Self::Totp
    }
}

/// Session status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[sqlx(type_name = "session_status", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SessionStatus {
    /// Session is active
    Active = 1,
    /// Session expired
    Expired = 2,
    /// Session revoked
    Revoked = 3,
    /// Session logged out
    LoggedOut = 4,
}

impl Default for SessionStatus {
    fn default() -> Self {
        Self::Active
    }
}

/// Login failure reason enum
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum LoginFailureReason {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Account locked")]
    AccountLocked,
    #[error("Account suspended")]
    AccountSuspended,
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
    #[error("IP blocked")]
    IpBlocked,
}

/// Registration error enum
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum RegistrationError {
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
    #[error("Registration disabled")]
    RegistrationDisabled,
}

/// Verification error enum
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum VerificationError {
    #[error("Document expired")]
    DocumentExpired,
    #[error("Document unreadable")]
    DocumentUnreadable,
    #[error("Verification pending")]
    VerificationPending,
    #[error("Verification failed")]
    VerificationFailed,
    #[error("Provider error")]
    ProviderError,
    #[error("Maximum attempts exceeded")]
    MaxAttemptsExceeded,
}

// #[cfg(test)]
// mod tests {
//     use super::*;

// #[test]
// fn test_user_role_levels() {
//     assert!(UserRole::SuperAdmin.level() > UserRole::Admin.level());
//     assert!(UserRole::Admin.level() > UserRole::Moderator.level());
//     assert!(UserRole::Moderator.level() > UserRole::Seller.level());
//     assert!(UserRole::Seller.level() > UserRole::Buyer.level());
//     assert!(UserRole::Buyer.level() > UserRole::Guest.level());
// }

//     #[test]
//     fn test_verification_level_progression() {
//         assert_eq!(
//             VerificationLevel::Level0.next_level(),
//             Some(VerificationLevel::Level1)
//         );
//         assert_eq!(
//             VerificationLevel::Level1.next_level(),
//             Some(VerificationLevel::Level2)
//         );
//         assert_eq!(
//             VerificationLevel::Level2.next_level(),
//             Some(VerificationLevel::Level3)
//         );
//         assert_eq!(
//             VerificationLevel::Level3.next_level(),
//             Some(VerificationLevel::Level4)
//         );
//         assert_eq!(VerificationLevel::Level4.next_level(), None);
//     }

//     #[test]
//     fn test_verification_level_requirements() {
//         assert!(!VerificationLevel::Level0.requires_document());
//         assert!(VerificationLevel::Level2.requires_document());

//         assert!(!VerificationLevel::Level0.requires_business_info());
//         assert!(VerificationLevel::Level3.requires_business_info());
//     }

//     #[test]
//     fn test_user_status_authentication() {
//         assert!(!UserStatus::Pending.can_authenticate());
//         assert!(UserStatus::Active.can_authenticate());
//         assert!(!UserStatus::Suspended.can_authenticate());
//         assert!(!UserStatus::Locked.can_authenticate());
//     }

//     #[test]
//     fn test_document_type_classification() {
//         assert!(DocumentType::Nin.is_personal_id());
//         assert!(!DocumentType::Nin.is_business_id());

//         assert!(DocumentType::CacCertificate.is_business_id());
//         assert!(!DocumentType::CacCertificate.is_personal_id());
//     }
// }
