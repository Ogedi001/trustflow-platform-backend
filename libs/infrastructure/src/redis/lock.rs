//! Distributed lock implementation for Redis
//!
//! Provides distributed locking capabilities using Redis SETNX for
//! coordinating access to shared resources across multiple instances.
//!
//! ## Feature Flags
//!
//! - `redis`: Enables Redis support (enabled by default with `full` feature)

#[cfg(feature = "redis")]
use async_trait::async_trait;

#[cfg(feature = "redis")]
use std::time::Duration;

#[cfg(feature = "redis")]
use super::{RedisError, RedisPool};
#[cfg(feature = "redis")]
use crate::redis::key::RedisKey;

/// Distributed lock trait
#[cfg(feature = "redis")]
#[async_trait]
pub trait DistributedLock: Send + Sync {
    /// Acquire a lock with the given key and TTL
    async fn acquire(&self, key: &str, ttl: Duration) -> Result<bool, RedisError>;

    /// Release a lock with the given key
    async fn release(&self, key: &str) -> Result<bool, RedisError>;

    /// Check if a lock exists
    async fn exists(&self, key: &str) -> Result<bool, RedisError>;
}

/// Redis-based distributed lock implementation
#[cfg(feature = "redis")]
#[derive(Clone)]
pub struct RedisLock {
    pool: RedisPool,
    prefix: String,
}

#[cfg(feature = "redis")]
impl RedisLock {
    /// Create a new Redis lock instance
    pub fn new(pool: RedisPool, prefix: impl Into<String>) -> Self {
        Self {
            pool,
            prefix: prefix.into(),
        }
    }

    /// Get the lock key
    fn lock_key(&self, resource: &str) -> RedisKey {
        RedisKey::lock(&self.prefix, resource)
    }
}

#[cfg(feature = "redis")]
#[async_trait]
impl DistributedLock for RedisLock {
    async fn acquire(&self, key: &str, ttl: Duration) -> Result<bool, RedisError> {
        let conn = self.pool.get_connection().await?;
        let lock_key = self.lock_key(key).as_str();

        // Use SETNX with PX for atomic lock acquisition
        let result: Option<String> = redis::cmd("SET")
            .arg(lock_key)
            .arg("locked")
            .arg("NX")
            .arg("PX")
            .arg(ttl.as_millis() as u64)
            .query_async(conn)
            .await
            .map_err(|e| RedisError::command("set", e.to_string()))?;

        Ok(result.is_some())
    }

    async fn release(&self, key: &str) -> Result<bool, RedisError> {
        let conn = self.pool.get_connection().await?;
        let lock_key = self.lock_key(key).as_str();

        // Use DEL to release the lock
        let deleted: u64 = redis::cmd("DEL")
            .arg(lock_key)
            .query_async(conn)
            .await
            .map_err(|e| RedisError::command("del", e.to_string()))?;

        Ok(deleted > 0)
    }

    async fn exists(&self, key: &str) -> Result<bool, RedisError> {
        let conn = self.pool.get_connection().await?;
        let lock_key = self.lock_key(key).as_str();

        let exists: u64 = redis::cmd("EXISTS")
            .arg(lock_key)
            .query_async(conn)
            .await
            .map_err(|e| RedisError::command("exists", e.to_string()))?;

        Ok(exists > 0)
    }
}
