//! Rate limiting for Redis infrastructure
//!
//! Provides distributed rate limiting using Redis as the backing store.
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

/// Rate limiter trait for distributed rate limiting
#[cfg(feature = "redis")]
#[async_trait]
pub trait RateLimiter: Send + Sync {
    /// Check if action is allowed and get remaining count
    async fn is_allowed(
        &self,
        key: &str,
        limit: u64,
        window: Duration,
    ) -> Result<(bool, u64), RedisError>;
    /// Get remaining count for a key
    async fn remaining(&self, key: &str, limit: u64, window: Duration) -> Result<u64, RedisError>;
    /// Reset rate limit for a key
    async fn reset(&self, key: &str) -> Result<(), RedisError>;
    /// Get current count for a key
    async fn current(&self, key: &str) -> Result<u64, RedisError>;
    /// Get TTL for a key
    async fn ttl(&self, key: &str) -> Result<i64, RedisError>;
}

/// Redis rate limiter implementation using sliding window algorithm
#[cfg(feature = "redis")]
#[derive(Clone)]
pub struct RedisRateLimiter {
    pool: RedisPool,
    prefix: String,
}

#[cfg(feature = "redis")]
impl RedisRateLimiter {
    /// Create a new Redis rate limiter
    pub fn new(pool: RedisPool, prefix: impl Into<String>) -> Self {
        Self {
            pool,
            prefix: prefix.into(),
        }
    }

    /// Get prefixed rate limit key
    fn rate_limit_key(&self, key: &str) -> RedisKey {
        RedisKey::rate_limit(&self.prefix, key)
    }
}

#[cfg(feature = "redis")]
#[async_trait]
impl RateLimiter for RedisRateLimiter {
    async fn is_allowed(
        &self,
        key: &str,
        limit: u64,
        window: Duration,
    ) -> Result<(bool, u64), RedisError> {
        let conn = self.pool.get_connection().await?;
        let prefixed_key = self.rate_limit_key(key);

        // Use sliding window with Lua script for atomicity
        let lua_script = r#"
            local key = KEYS[1]
            local limit = tonumber(ARGV[1])
            local window = tonumber(ARGV[2])
            local now = tonumber(ARGV[3])
            
            -- Remove old entries outside the window
            redis.call('ZREMRANGEBYSCORE', key, '-inf', now - window * 1000)
            
            -- Count current requests
            local count = redis.call('ZCARD', key)
            
            if count < limit then
                -- Add new request
                redis.call('ZADD', key, now, now .. ':' .. math.random(1000000))
                redis.call('EXPIRE', key, window)
                return {1, limit - count - 1}
            else
                return {0, 0}
            end
        "#;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let result: Vec<String> = redis::cmd("EVAL")
            .arg(lua_script)
            .arg(1)
            .arg(prefixed_key.as_str())
            .arg(limit)
            .arg(window.as_secs())
            .arg(now)
            .query_async(conn)
            .await
            .map_err(|e| RedisError::command("redis", e.to_string()))?;

        if result.len() >= 2 {
            let allowed: u64 = result[0].parse().unwrap_or(0);
            let remaining: u64 = result[1].parse().unwrap_or(0);
            Ok((allowed == 1, remaining))
        } else {
            Ok((false, 0))
        }
    }

    async fn remaining(&self, key: &str, limit: u64, window: Duration) -> Result<u64, RedisError> {
        let conn = self.pool.get_connection().await?;
        let prefixed_key = self.rate_limit_key(key);

        let lua_script = r#"
            local key = KEYS[1]
            local limit = tonumber(ARGV[1])
            local window = tonumber(ARGV[2])
            local now = tonumber(ARGV[3])
            
            -- Remove old entries
            redis.call('ZREMRANGEBYSCORE', key, '-inf', now - window * 1000)
            
            local count = redis.call('ZCARD', key)
            
            if count < limit then
                return limit - count
            else
                return 0
            end
        "#;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        let remaining: u64 = redis::cmd("EVAL")
            .arg(lua_script)
            .arg(1)
            .arg(prefixed_key.as_str())
            .arg(limit)
            .arg(window.as_secs())
            .arg(now)
            .query_async(conn)
            .await
            .map_err(|e| RedisError::command("redis", e.to_string()))?;

        Ok(remaining)
    }

    async fn reset(&self, key: &str) -> Result<(), RedisError> {
        let conn = self.pool.get_connection().await?;

        redis::cmd("DEL")
            .arg(self.rate_limit_key(key).as_str())
            .query_async::<_, u64>(conn)
            .await
            .map_err(|e| RedisError::command("redis", e.to_string()))?;

        Ok(())
    }

    async fn current(&self, key: &str) -> Result<u64, RedisError> {
        let conn = self.pool.get_connection().await?;

        let count: u64 = redis::cmd("ZCARD")
            .arg(self.rate_limit_key(key).as_str())
            .query_async(conn)
            .await
            .map_err(|e| RedisError::command("redis", e.to_string()))?;

        Ok(count)
    }

    async fn ttl(&self, key: &str) -> Result<i64, RedisError> {
        let conn = self.pool.get_connection().await?;

        let ttl: i64 = redis::cmd("TTL")
            .arg(self.rate_limit_key(key).as_str())
            .query_async(conn)
            .await
            .map_err(|e| RedisError::command("redis", e.to_string()))?;

        Ok(ttl)
    }
}

/// Fixed window rate limiter (simpler, less accurate)
#[cfg(feature = "redis")]
#[derive(Clone)]
pub struct RedisFixedWindowRateLimiter {
    pool: RedisPool,
    prefix: String,
}

#[cfg(feature = "redis")]
impl RedisFixedWindowRateLimiter {
    /// Create a new fixed window rate limiter
    pub fn new(pool: RedisPool, prefix: impl Into<String>) -> Self {
        Self {
            pool,
            prefix: prefix.into(),
        }
    }

    /// Get prefixed rate limit key with window
    fn rate_limit_key(&self, key: &str, window: u64) -> RedisKey {
        // calculate current window index
        let window_id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            / window;
        RedisKey::from_parts([&self.prefix, "ratelimit", key, &window_id.to_string()])
    }
}

#[cfg(feature = "redis")]
#[async_trait]
impl RateLimiter for RedisFixedWindowRateLimiter {
    async fn is_allowed(
        &self,
        key: &str,
        limit: u64,
        window: Duration,
    ) -> Result<(bool, u64), RedisError> {
        let conn = self.pool.get_connection().await?;
        let prefixed_key = self.rate_limit_key(key, window.as_secs());

        let count: u64 = redis::cmd("INCR")
            .arg(&prefixed_key)
            .query_async(conn.clone())
            .await
            .map_err(|e| RedisError::command("redis", e.to_string()))?;

        if count == 1 {
            // First request, set expiry
            let mut cmd = redis::cmd("EXPIRE");
            cmd.arg(&prefixed_key).arg(window.as_secs());
            cmd.query_async::<_, bool>(conn)
                .await
                .map_err(|e| RedisError::command("redis", e.to_string()))?;
        }

        let remaining = if count > limit { 0 } else { limit - count + 1 };
        Ok((count <= limit, remaining))
    }

    async fn remaining(&self, key: &str, limit: u64, window: Duration) -> Result<u64, RedisError> {
        let conn = self.pool.get_connection().await?;
        let prefixed_key = self.rate_limit_key(key, window.as_secs());

        let count: u64 = redis::cmd("GET")
            .arg(&prefixed_key)
            .query_async(conn)
            .await
            .map_err(|e| RedisError::command("redis", e.to_string()))?;

        let remaining = if count >= limit { 0 } else { limit - count };
        Ok(remaining)
    }

    async fn reset(&self, key: &str) -> Result<(), RedisError> {
        let conn = self.pool.get_connection().await?;

        // For fixed window, we can't easily know all window keys
        // This is a limitation of the fixed window algorithm
        let pattern = format!("{}:ratelimit:{}:*", self.prefix, key);
        redis::cmd("DEL")
            .arg(pattern)
            .query_async::<_, u64>(conn)
            .await
            .map_err(|e| RedisError::command("redis", e.to_string()))?;

        Ok(())
    }

    async fn current(&self, key: &str) -> Result<u64, RedisError> {
        let conn = self.pool.get_connection().await?;
        let prefixed_key = self.rate_limit_key(key, 60); // Default to 60s window

        let count: u64 = redis::cmd("GET")
            .arg(&prefixed_key)
            .query_async(conn)
            .await
            .map_err(|e| RedisError::command("redis", e.to_string()))?;

        Ok(count)
    }

    async fn ttl(&self, key: &str) -> Result<i64, RedisError> {
        let conn = self.pool.get_connection().await?;

        let pattern = format!("{}:ratelimit:{}:*", self.prefix, key);
        let ttl: i64 = redis::cmd("TTL")
            .arg(pattern)
            .query_async(conn)
            .await
            .map_err(|e| RedisError::command("redis", e.to_string()))?;

        Ok(ttl)
    }
}
