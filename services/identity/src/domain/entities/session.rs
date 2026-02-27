use serde::{Deserialize, Serialize};
use uuid::Uuid;


/// Session entity for managing user sessions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Session {
    pub id: SessionId,
    pub user_id: UserId,
    pub device_id: DeviceId,
    pub user_agent: String,
    pub ip_address: IpAddress,
    pub token_hash: String,
    pub refresh_token_hash: String,
    pub expires_at: Timestamp,
    pub refresh_expires_at: Timestamp,
    pub revoked: bool,
    pub created_at: Timestamp,
    pub last_activity_at: Timestamp,
}

impl Session {
    pub fn new(
        user_id: UserId,
        device_id: DeviceId,
        user_agent: String,
        ip_address: IpAddress,
        token_hash: String,
        refresh_token_hash: String,
        expires_at: Timestamp,
        refresh_expires_at: Timestamp,
    ) -> Self {
        Self {
            id: SessionId::new(),
            user_id,
            device_id,
            user_agent,
            ip_address,
            token_hash,
            refresh_token_hash,
            expires_at,
            refresh_expires_at,
            revoked: false,
            created_at: Timestamp::now(),
            last_activity_at: Timestamp::now(),
        }
    }

    /// Check if session is valid
    pub fn is_valid(&self) -> bool {
        !self.revoked && self.expires_at.inner() > time::OffsetDateTime::now_utc()
    }

    /// Revoke session
    pub fn revoke(&mut self) {
        self.revoked = true;
    }
}

/// Session ID newtype
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(pub Uuid);

impl SessionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}