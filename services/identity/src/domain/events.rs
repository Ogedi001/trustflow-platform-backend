//! Domain events for Identity Service
//!
//! Events published for inter-service communication.

use crate::domain::entities::*;
use crate::domain::enums::*;
use common::Timestamp;
use serde::{Deserialize, Serialize};

/// Base event trait
pub trait DomainEvent: Send + Sync {
    fn event_type(&self) -> &str;
    fn timestamp(&self) -> Timestamp;
    fn aggregate_id(&self) -> String;
}

/// User registered event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegisteredEvent {
    pub user_id: UserId,
    pub email: EmailAddress,
    pub phone: PhoneNumber,
    pub role: UserRole,
    pub timestamp: Timestamp,
}

impl DomainEvent for UserRegisteredEvent {
    fn event_type(&self) -> &str {
        "user.registered"
    }

    fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    fn aggregate_id(&self) -> String {
        self.user_id.0.to_string()
    }
}

/// Email verified event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailVerifiedEvent {
    pub user_id: UserId,
    pub email: EmailAddress,
    pub timestamp: Timestamp,
}

impl DomainEvent for EmailVerifiedEvent {
    fn event_type(&self) -> &str {
        "user.email_verified"
    }

    fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    fn aggregate_id(&self) -> String {
        self.user_id.0.to_string()
    }
}

/// User logged in event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLoggedInEvent {
    pub user_id: UserId,
    pub session_id: SessionId,
    pub device_id: DeviceId,
    pub ip_address: IpAddress,
    pub timestamp: Timestamp,
}

impl DomainEvent for UserLoggedInEvent {
    fn event_type(&self) -> &str {
        "user.logged_in"
    }

    fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    fn aggregate_id(&self) -> String {
        self.user_id.0.to_string()
    }
}

/// User logged out event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLoggedOutEvent {
    pub user_id: UserId,
    pub session_id: SessionId,
    pub reason: LogoutReason,
    pub timestamp: Timestamp,
}

impl DomainEvent for UserLoggedOutEvent {
    fn event_type(&self) -> &str {
        "user.logged_out"
    }

    fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    fn aggregate_id(&self) -> String {
        self.user_id.0.to_string()
    }
}

/// Password changed event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordChangedEvent {
    pub user_id: UserId,
    pub reason: PasswordChangeReason,
    pub timestamp: Timestamp,
}

impl DomainEvent for PasswordChangedEvent {
    fn event_type(&self) -> &str {
        "user.password_changed"
    }

    fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    fn aggregate_id(&self) -> String {
        self.user_id.0.to_string()
    }
}

/// MFA enabled event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaEnabledEvent {
    pub user_id: UserId,
    pub method: String,
    pub timestamp: Timestamp,
}

impl DomainEvent for MfaEnabledEvent {
    fn event_type(&self) -> &str {
        "user.mfa_enabled"
    }

    fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    fn aggregate_id(&self) -> String {
        self.user_id.0.to_string()
    }
}

/// MFA disabled event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaDisabledEvent {
    pub user_id: UserId,
    pub reason: String,
    pub timestamp: Timestamp,
}

impl DomainEvent for MfaDisabledEvent {
    fn event_type(&self) -> &str {
        "user.mfa_disabled"
    }

    fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    fn aggregate_id(&self) -> String {
        self.user_id.0.to_string()
    }
}

/// Account suspended event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSuspendedEvent {
    pub user_id: UserId,
    pub reason: String,
    pub suspended_by: UserId,
    pub timestamp: Timestamp,
}

impl DomainEvent for AccountSuspendedEvent {
    fn event_type(&self) -> &str {
        "user.suspended"
    }

    fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    fn aggregate_id(&self) -> String {
        self.user_id.0.to_string()
    }
}

/// Account activated event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountActivatedEvent {
    pub user_id: UserId,
    pub activated_by: UserId,
    pub timestamp: Timestamp,
}

impl DomainEvent for AccountActivatedEvent {
    fn event_type(&self) -> &str {
        "user.activated"
    }

    fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    fn aggregate_id(&self) -> String {
        self.user_id.0.to_string()
    }
}

/// Verification level updated event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationLevelUpdatedEvent {
    pub user_id: UserId,
    pub old_level: VerificationLevel,
    pub new_level: VerificationLevel,
    pub timestamp: Timestamp,
}

impl DomainEvent for VerificationLevelUpdatedEvent {
    fn event_type(&self) -> &str {
        "user.verification_level_updated"
    }

    fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    fn aggregate_id(&self) -> String {
        self.user_id.0.to_string()
    }
}

/// Verification approved event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationApprovedEvent {
    pub user_id: UserId,
    pub verification_id: VerificationId,
    pub level: VerificationLevel,
    pub approved_by: UserId,
    pub timestamp: Timestamp,
}

impl DomainEvent for VerificationApprovedEvent {
    fn event_type(&self) -> &str {
        "verification.approved"
    }

    fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    fn aggregate_id(&self) -> String {
        self.user_id.0.to_string()
    }
}

/// Verification rejected event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationRejectedEvent {
    pub user_id: UserId,
    pub verification_id: VerificationId,
    pub level: VerificationLevel,
    pub reason: String,
    pub rejected_by: UserId,
    pub timestamp: Timestamp,
}

impl DomainEvent for VerificationRejectedEvent {
    fn event_type(&self) -> &str {
        "verification.rejected"
    }

    fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    fn aggregate_id(&self) -> String {
        self.user_id.0.to_string()
    }
}

/// Role assigned event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleAssignedEvent {
    pub user_id: UserId,
    pub role_id: RoleId,
    pub role_name: RoleName,
    pub assigned_by: UserId,
    pub timestamp: Timestamp,
}

impl DomainEvent for RoleAssignedEvent {
    fn event_type(&self) -> &str {
        "user.role_assigned"
    }

    fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    fn aggregate_id(&self) -> String {
        self.user_id.0.to_string()
    }
}

/// Role removed event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleRemovedEvent {
    pub user_id: UserId,
    pub role_id: RoleId,
    pub role_name: RoleName,
    pub removed_by: UserId,
    pub timestamp: Timestamp,
}

impl DomainEvent for RoleRemovedEvent {
    fn event_type(&self) -> &str {
        "user.role_removed"
    }

    fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    fn aggregate_id(&self) -> String {
        self.user_id.0.to_string()
    }
}

/// Session revoked event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRevokedEvent {
    pub user_id: UserId,
    pub session_id: SessionId,
    pub reason: RevocationReason,
    pub timestamp: Timestamp,
}

impl DomainEvent for SessionRevokedEvent {
    fn event_type(&self) -> &str {
        "session.revoked"
    }

    fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    fn aggregate_id(&self) -> String {
        self.user_id.0.to_string()
    }
}

/// Suspicious activity detected event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspiciousActivityEvent {
    pub user_id: UserId,
    pub activity_type: SuspiciousActivityType,
    pub details: String,
    pub ip_address: IpAddress,
    pub timestamp: Timestamp,
}

impl DomainEvent for SuspiciousActivityEvent {
    fn event_type(&self) -> &str {
        "security.suspicious_activity"
    }

    fn timestamp(&self) -> Timestamp {
        self.timestamp
    }

    fn aggregate_id(&self) -> String {
        self.user_id.0.to_string()
    }
}

/// Logout reason enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogoutReason {
    UserInitiated,
    SessionExpired,
    SecurityViolation,
    AdminRevoked,
    PasswordChanged,
    AccountDeleted,
    DeviceLost,
}

/// Password change reason enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PasswordChangeReason {
    UserInitiated,
    SecurityBreach,
    AdminInitiated,
    Expiration,
}

/// Session revocation reason enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RevocationReason {
    UserLogout,
    AllSessionsLogout,
    SecurityViolation,
    SessionExpired,
    AdminRevoked,
    DeviceCompromised,
}

/// Suspicious activity type enum
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SuspiciousActivityType {
    MultipleFailedLogins,
    UnusualLocation,
    UnusualTime,
    NewDevice,
    MultipleAccountsSameDevice,
    BruteForceAttempt,
    CredentialStuffing,
    AnomalousBehavior,
}

/// Event publisher trait for publishing domain events
#[async_trait::async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish(
        &self,
        event: &dyn DomainEvent,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

/// Null event publisher for testing
pub struct NullEventPublisher;

#[async_trait::async_trait]
impl EventPublisher for NullEventPublisher {
    async fn publish(
        &self,
        _event: &dyn DomainEvent,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
}
