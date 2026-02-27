//! Generic caching for Redis infrastructure
//!
//! Provides a generic cache interface for storing and retrieving serializable data.
//!
//! ## Feature Flags
//!
//! - `redis`: Enables Redis support (enabled by default with `full` feature)

#[cfg(feature = "redis")]
use async_trait::async_trait;

#[cfg(feature = "redis")]
use serde::{Serialize, de::DeserializeOwned};

#[cfg(feature = "redis")]
use std::time::Duration;

#[cfg(feature = "redis")]
use super::{RedisError, RedisPool};
#[cfg(feature = "redis")]
use crate::redis::key::RedisKey;

/// Cache trait for generic caching operations
#[cfg(feature = "redis")]
#[async_trait]
pub trait Cache: Send + Sync {
    /// Get a value from cache
    async fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, RedisError>;
    /// Set a value in cache with TTL
    async fn set<T: Serialize>(
        &self,
        key: &str,
        value: &T,
        ttl: Duration,
    ) -> Result<(), RedisError>;
    /// Delete a value from cache
    async fn delete(&self, key: &str) -> Result<(), RedisError>;
    /// Check if a key exists
    async fn exists(&self, key: &str) -> Result<bool, RedisError>;
    /// Get TTL of a key
    async fn ttl(&self, key: &str) -> Result<Option<i64>, RedisError>;
    /// Increment a counter
    async fn increment(&self, key: &str, amount: i64) -> Result<i64, RedisError>;
    /// Get multiple values
    async fn get_many<T: DeserializeOwned>(
        &self,
        keys: &[&str],
    ) -> Result<Vec<Option<T>>, RedisError>;
    /// Delete multiple values
    async fn delete_many(&self, keys: &[&str]) -> Result<u64, RedisError>;
}

/// Redis cache implementation
#[cfg(feature = "redis")]
#[derive(Clone)]
pub struct RedisCache {
    pool: RedisPool,
    prefix: String,
}

#[cfg(feature = "redis")]
impl RedisCache {
    /// Create a new Redis cache
    pub fn new(pool: RedisPool, prefix: impl Into<String>) -> Self {
        Self {
            pool,
            prefix: prefix.into(),
        }
    }

    /// Access the prefix that was supplied at construction.
    ///
    /// Other components (e.g. OTP) can use this in combination with
    /// `RedisKey` to build domain‑specific keys without poking into the
    /// struct's internals.
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// Get prefixed key using `RedisKey` builder
    fn key(&self, key: &str) -> RedisKey {
        RedisKey::cache(&self.prefix, key)
    }
}

#[cfg(feature = "redis")]
#[async_trait]
impl Cache for RedisCache {
    async fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, RedisError> {
        let conn = self.pool.get_connection().await?;

        let data: Option<String> = redis::cmd("GET")
            .arg(self.key(key).as_str())
            .query_async(conn)
            .await
            .map_err(|e| RedisError::command("redis", e.to_string()))?;

        match data {
            Some(json) => {
                let value = serde_json::from_str(&json)
                    .map_err(|e| RedisError::deserialization("JSON", e.to_string()))?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    async fn set<T: Serialize>(
        &self,
        key: &str,
        value: &T,
        ttl: Duration,
    ) -> Result<(), RedisError> {
        let conn = self.pool.get_connection().await?;
        let data = serde_json::to_string(value)
            .map_err(|e| RedisError::serialization("JSON", e.to_string()))?;

        let mut cmd = redis::cmd("SET");
        cmd.arg(self.key(key).as_str())
            .arg(data)
            .arg("EX")
            .arg(ttl.as_secs());

        cmd.query_async::<_, String>(conn)
            .await
            .map_err(|e| RedisError::command("redis", e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<(), RedisError> {
        let conn = self.pool.get_connection().await?;

        redis::cmd("DEL")
            .arg(self.key(key).as_str())
            .query_async::<_, u64>(conn)
            .await
            .map_err(|e| RedisError::command("redis", e.to_string()))?;

        Ok(())
    }

    async fn exists(&self, key: &str) -> Result<bool, RedisError> {
        let conn = self.pool.get_connection().await?;

        let result: u64 = redis::cmd("EXISTS")
            .arg(self.key(key).as_str())
            .query_async(conn)
            .await
            .map_err(|e| RedisError::command("redis", e.to_string()))?;

        Ok(result > 0)
    }

    async fn ttl(&self, key: &str) -> Result<Option<i64>, RedisError> {
        let conn = self.pool.get_connection().await?;

        let ttl: i64 = redis::cmd("TTL")
            .arg(self.key(key).as_str())
            .query_async(conn)
            .await
            .map_err(|e| RedisError::command("redis", e.to_string()))?;

        if ttl == -2 {
            Ok(None) // Key doesn't exist
        } else {
            Ok(Some(ttl))
        }
    }

    async fn increment(&self, key: &str, amount: i64) -> Result<i64, RedisError> {
        let conn = self.pool.get_connection().await?;

        let result: i64 = redis::cmd("INCRBY")
            .arg(self.key(key).as_str())
            .arg(amount)
            .query_async(conn)
            .await
            .map_err(|e| RedisError::command("redis", e.to_string()))?;

        Ok(result)
    }

    async fn get_many<T: DeserializeOwned>(
        &self,
        keys: &[&str],
    ) -> Result<Vec<Option<T>>, RedisError> {
        let conn = self.pool.get_connection().await?;

        let mut cmd = redis::cmd("MGET");
        for key in keys {
            cmd.arg(self.key(key).as_str());
        }

        let data: Vec<Option<String>> = cmd
            .query_async(conn)
            .await
            .map_err(|e| RedisError::command("redis", e.to_string()))?;

        let mut results = Vec::new();
        for item in data {
            match item {
                Some(json) => {
                    let value = serde_json::from_str(&json)
                        .map_err(|e| RedisError::deserialization("JSON", e.to_string()))?;
                    results.push(Some(value));
                }
                None => results.push(None),
            }
        }

        Ok(results)
    }

    async fn delete_many(&self, keys: &[&str]) -> Result<u64, RedisError> {
        if keys.is_empty() {
            return Ok(0);
        }

        let conn = self.pool.get_connection().await?;

        let mut cmd = redis::cmd("DEL");
        for key in keys {
            cmd.arg(self.key(key).as_str());
        }

        let deleted: u64 = cmd
            .query_async(conn)
            .await
            .map_err(|e| RedisError::command("redis", e.to_string()))?;

        Ok(deleted)
    }
}
