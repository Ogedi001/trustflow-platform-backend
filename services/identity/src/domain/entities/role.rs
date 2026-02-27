use crate::domain::value_objects::Permission;
use common::Timestamp;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Role entity (One Role -> Many Users)
pub struct Role {
    pub id: RoleId,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub permissions: Vec<Permission>,
    pub role_level: i32,
    pub is_active: bool,
    pub is_system_role: bool,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RoleId(pub Uuid);

impl RoleId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Individual permission entry
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Permission {
    pub resource: String,
    pub action: String,
    #[serde(default)]
    pub conditions: Option<serde_json::Value>,
}

/// Permissions list type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Permissions(pub Vec<Permission>);
