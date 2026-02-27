//! Session storage for Redis infrastructure
//!
//! Provides session management using Redis as the backing store.
//!
//! ## Feature Flags
//!
//! - `redis`: Enables Redis support (enabled by default with `full` feature)

#[cfg(feature = "redis")]
use async_trait::async_trait;

#[cfg(feature = "redis")]
use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[cfg(feature = "redis")]
use std::time::Duration;

#[cfg(feature = "redis")]
use super::{RedisError, RedisPool};
#[cfg(feature = "redis")]
use crate::redis::key::RedisKey;

/// Session data structure
#[cfg(feature = "redis")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub user_id: String,
    pub email: String,
    pub role: String,
    pub session_id: String,
    pub device_id: String,
    pub user_agent: String,
    pub ip_address: String,
    pub created_at: String,
    pub last_activity: String,
}

/// Session store trait
#[cfg(feature = "redis")]
#[async_trait]
pub trait SessionStore: Send + Sync {
    /// Save session with TTL
    async fn save_session(
        &self,
        key: &str,
        session: &SessionData,
        ttl: Duration,
    ) -> Result<(), RedisError>;
    /// Get session by key
    async fn get_session(&self, key: &str) -> Result<Option<SessionData>, RedisError>;
    /// Delete session
    async fn delete_session(&self, key: &str) -> Result<(), RedisError>;
    /// Update session activity (refresh TTL)
    async fn update_activity(&self, key: &str) -> Result<(), RedisError>;
    /// Delete all sessions for a user
    async fn delete_user_sessions(&self, user_id: &str) -> Result<u64, RedisError>;
    /// Get all sessions for a user
    async fn get_user_sessions(&self, user_id: &str) -> Result<Vec<SessionData>, RedisError>;
}

/// Redis session store implementation
#[cfg(feature = "redis")]
#[derive(Clone)]
pub struct RedisSessionStore {
    pool: RedisPool,
    prefix: String,
}

#[cfg(feature = "redis")]
impl RedisSessionStore {
    /// Create a new Redis session store
    pub fn new(pool: RedisPool, prefix: impl Into<String>) -> Self {
        Self {
            pool,
            prefix: prefix.into(),
        }
    }

    /// Get prefixed session key
    fn session_key(&self, session_id: &str) -> RedisKey {
        RedisKey::session(&self.prefix, session_id)
    }

    /// Get prefixed user sessions key (for listing user sessions)
    fn user_sessions_key(&self, user_id: &str) -> RedisKey {
        RedisKey::user_sessions(&self.prefix, user_id)
    }
}

#[cfg(feature = "redis")]
#[async_trait]
impl SessionStore for RedisSessionStore {
    async fn save_session(
        &self,
        key: &str,
        session: &SessionData,
        ttl: Duration,
    ) -> Result<(), RedisError> {
        let conn = self.pool.get_connection().await?;
        let data = serde_json::to_string(session)
            .map_err(|e| RedisError::serialization("JSON", e.to_string()))?;

        // Save session data
        let mut cmd = redis::cmd("SET");
        cmd.arg(self.session_key(key).as_str())
            .arg(data)
            .arg("EX")
            .arg(ttl.as_secs());

        cmd.query_async::<_, String>(conn.clone())
            .await
            .map_err(|e| RedisError::command("redis", e.to_string()))?;

        // Add to user's session set
        let mut cmd = redis::cmd("SADD");
        cmd.arg(self.user_sessions_key(&session.user_id).as_str())
            .arg(key);

        cmd.query_async::<_, u64>(conn)
            .await
            .map_err(|e| RedisError::command("redis", e.to_string()))?;

        Ok(())
    }

    async fn get_session(&self, key: &str) -> Result<Option<SessionData>, RedisError> {
        let conn = self.pool.get_connection().await?;

        let data: Option<String> = redis::cmd("GET")
            .arg(self.session_key(key).as_str())
            .query_async(conn)
            .await
            .map_err(|e| RedisError::command("redis", e.to_string()))?;

        match data {
            Some(json) => {
                let session = serde_json::from_str(&json)
                    .map_err(|e| RedisError::deserialization("JSON", e.to_string()))?;
                Ok(Some(session))
            }
            None => Ok(None),
        }
    }

    async fn delete_session(&self, key: &str) -> Result<(), RedisError> {
        let conn = self.pool.get_connection().await?;

        // Get session data first to remove from user sessions set
        let session = self.get_session(key).await?;

        if let Some(s) = session {
            let mut cmd = redis::cmd("SREM");
            cmd.arg(self.user_sessions_key(&s.user_id).as_str())
                .arg(key);

            cmd.query_async::<_, u64>(conn.clone())
                .await
                .map_err(|e| RedisError::command("redis", e.to_string()))?;
        }

        // Delete session data
        redis::cmd("DEL")
            .arg(self.session_key(key).as_str())
            .query_async::<_, u64>(conn)
            .await
            .map_err(|e| RedisError::command("redis", e.to_string()))?;

        Ok(())
    }

    async fn update_activity(&self, key: &str) -> Result<(), RedisError> {
        let conn = self.pool.get_connection().await?;

        redis::cmd("EXPIRE")
            .arg(self.session_key(key).as_str())
            .arg(86400) // 24 hours TTL
            .query_async::<_, u64>(conn)
            .await
            .map_err(|e| RedisError::command("redis", e.to_string()))?;

        Ok(())
    }

    async fn delete_user_sessions(&self, user_id: &str) -> Result<u64, RedisError> {
        let conn = self.pool.get_connection().await?;

        // Get all session IDs for this user
        let session_ids: Vec<String> = redis::cmd("SMEMBERS")
            .arg(self.user_sessions_key(user_id).as_str())
            .query_async(conn.clone())
            .await
            .map_err(|e| RedisError::command("redis", e.to_string()))?;

        let mut deleted_count = 0;
        for session_id in &session_ids {
            redis::cmd("DEL")
                .arg(self.session_key(session_id).as_str())
                .query_async::<_, u64>(conn.clone())
                .await
                .map_err(|e| RedisError::command("redis", e.to_string()))?;
            deleted_count += 1;
        }

        // Delete the user's session set
        redis::cmd("DEL")
            .arg(self.user_sessions_key(user_id).as_str())
            .query_async::<_, u64>(conn)
            .await
            .map_err(|e| RedisError::command("redis", e.to_string()))?;

        Ok(deleted_count)
    }

    async fn get_user_sessions(&self, user_id: &str) -> Result<Vec<SessionData>, RedisError> {
        let conn = self.pool.get_connection().await?;

        // Get all session IDs for this user
        let session_ids: Vec<String> = redis::cmd("SMEMBERS")
            .arg(self.user_sessions_key(user_id).as_str())
            .query_async(conn)
            .await
            .map_err(|e| RedisError::command("redis", e.to_string()))?;

        let mut sessions = Vec::new();
        for session_id in session_ids {
            if let Some(session) = self.get_session(&session_id).await? {
                sessions.push(session);
            }
        }

        Ok(sessions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_session_data_serde() {
        let session = SessionData {
            user_id: "user-123".to_string(),
            email: "test@example.com".to_string(),
            role: "BUYER".to_string(),
            session_id: "sess-456".to_string(),
            device_id: "device-789".to_string(),
            user_agent: "Mozilla/5.0".to_string(),
            ip_address: "192.168.1.1".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            last_activity: "2024-01-01T00:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&session).unwrap();
        let decoded: SessionData = serde_json::from_str(&json).unwrap();

        assert_eq!(session.user_id, decoded.user_id);
        assert_eq!(session.email, decoded.email);
        assert_eq!(session.role, decoded.role);
    }
}
