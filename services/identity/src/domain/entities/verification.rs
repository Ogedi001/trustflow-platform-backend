use serde::{Deserialize, Serialize};
use uuid::Uuid;

impl Metadata {
    pub fn insert(&mut self, key: &str, value: &str) {
        let value = serde_json::Value::String(value.to_string());
        if let serde_json::Value::Object(map) = &mut self.0 {
            map.insert(key.to_string(), value);
        }
    }
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        if let serde_json::Value::Object(map) = &self.0 {
            map.get(key)
        } else {
            None
        }
    }
}

/// Verification record entity
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationRecord {
    pub id: VerificationId,
    pub user_id: UserId,
    pub level: VerificationLevel,
    pub status: VerificationStatus,
    pub method: VerificationMethod,
    pub document_type: Option<DocumentType>,
    pub document_url: Option<Url>,
    pub document_hash: Option<String>,
    pub verified_by: Option<UserId>,
    pub verified_at: Option<Timestamp>,
    pub expires_at: Option<Timestamp>,
    pub rejection_reason: Option<String>,
    pub metadata: Metadata,
    pub created_at: Timestamp,
}

impl VerificationRecord {
    pub fn new(user_id: UserId, level: VerificationLevel, method: VerificationMethod) -> Self {
        Self {
            id: VerificationId::new(),
            user_id,
            level,
            status: VerificationStatus::Pending,
            method,
            document_type: None,
            document_url: None,
            document_hash: None,
            verified_by: None,
            verified_at: None,
            expires_at: None,
            rejection_reason: None,
            metadata: Metadata::default(),
            created_at: Timestamp::now(),
        }
    }

    /// Approve verification
    pub fn approve(&mut self, approved_by: UserId) {
        self.status = VerificationStatus::Approved;
        self.verified_by = Some(approved_by);
        self.verified_at = Some(Timestamp::now());
    }

    /// Reject verification
    pub fn reject(&mut self, reason: &str) {
        self.status = VerificationStatus::Rejected;
        self.rejection_reason = Some(reason.to_string());
    }
}

/// Verification ID newtype
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VerificationId(pub Uuid);

impl VerificationId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}
