//! Identity-related value objects
//!
//! This module contains value objects related to user and resource identification.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::Hash;
use std::str::FromStr;

/// User ID newtype wrapper for type safety across services
///
/// Provides strong typing for user identifiers throughout the system.
/// Internally uses UUID v4 for distributed uniqueness.
///
/// # Example
///
/// ```rust
/// use common::value_objects::UserId;
///
/// let user_id = UserId::new();
/// assert_eq!(user_id, user_id.clone());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub uuid::Uuid);

impl UserId {
    /// Generate a new random UserId
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    /// Create from an existing UUID
    pub fn from_uuid(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }

    /// Get the underlying UUID
    pub fn as_uuid(&self) -> uuid::Uuid {
        self.0
    }

    /// Convert to string representation
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<uuid::Uuid> for UserId {
    fn from(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }
}

impl FromStr for UserId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        uuid::Uuid::parse_str(s).map(Self)
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Generic ID wrapper for any service resource
///
/// Use this for IDs of resources other than users (orders, disputes, etc.)
/// when you want type safety but don't need a specific type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResourceId(pub uuid::Uuid);

impl ResourceId {
    /// Generate a new random ResourceId
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    /// Get the underlying UUID
    pub fn as_uuid(&self) -> uuid::Uuid {
        self.0
    }
}

impl Default for ResourceId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<uuid::Uuid> for ResourceId {
    fn from(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }
}

impl fmt::Display for ResourceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Device ID wrapper for tracking devices accessing the system
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeviceId(pub String);

impl DeviceId {
    /// Create from string
    pub fn new(id: String) -> Self {
        Self(id)
    }

    /// Get the device ID value
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DeviceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id_new() {
        let user_id = UserId::new();
        let user_id2 = UserId::new();
        assert_ne!(user_id, user_id2);
    }

    #[test]
    fn test_user_id_from_str() {
        let uuid_str = "123e4567-e89b-12d3-a456-426614174000";
        let user_id: UserId = uuid_str.parse().unwrap();
        assert_eq!(user_id.to_string(), uuid_str);
    }

    #[test]
    fn test_resource_id_round_trip() {
        let resource_id = ResourceId::new();
        let uuid = resource_id.as_uuid();
        let resource_id2 = ResourceId::from(uuid);
        assert_eq!(resource_id, resource_id2);
    }
}
