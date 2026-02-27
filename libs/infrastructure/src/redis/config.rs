//! Redis configuration
//!
//! Centralized Redis configuration for the modular monolith.
//! All domains (cache, sessions, rate limiting, pubsub, OTP, etc.)
//! share the same Redis instance but can define domain-specific behavior.
//!
//! This design avoids premature microservice complexity while remaining
//! future-ready for extraction into separate services if needed.

use config::core::error::{ConfigError, ConfigResult};
use config::loader::ConfigLoader;
use serde::{Deserialize, Serialize};
use time::Duration;
use url::Url;

/// Redis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    /// Redis connection URL
    pub url: String,
    /// Key prefix for all Redis keys
    pub key_prefix: String,
    /// Maximum number of connections in the pool
    pub max_connections: u32,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Command timeout
    pub command_timeout: Duration,
    /// Connection retry delay
    pub retry_delay: Duration,
    /// Domain-specific settings
    pub domains: RedisDomainsConfig,
}

/// Domain-specific Redis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisDomainsConfig {
    pub session: SessionConfig,
    pub rate_limit: RateLimitConfig,
    pub cache: CacheConfig,
}

/// Session behavior configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Default session TTL
    pub ttl: Duration,

    /// Maximum active sessions per user
    pub max_sessions_per_user: u32,

    /// Refresh threshold (percentage of TTL)
    pub refresh_threshold: f32,
}

/// Rate limiting behavior configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Default window duration
    pub window: Duration,

    /// Default requests allowed per window
    pub default_limit: u64,
}

/// Cache behavior configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Default cache TTL
    pub ttl: Duration,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://localhost:6379".to_string(),
            key_prefix: "app".to_string(),
            max_connections: 50,
            connection_timeout: Duration::seconds(10),
            command_timeout: Duration::seconds(5),
            retry_delay: Duration::milliseconds(100),
            domains: RedisDomainsConfig::default(),
        }
    }
}

impl Default for RedisDomainsConfig {
    fn default() -> Self {
        Self {
            session: SessionConfig::default(),
            rate_limit: RateLimitConfig::default(),
            cache: CacheConfig::default(),
        }
    }
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            ttl: Duration::days(7),
            max_sessions_per_user: 5,
            refresh_threshold: 0.8,
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            window: Duration::minutes(15),
            default_limit: 100,
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            ttl: Duration::minutes(10),
        }
    }
}
impl RedisConfig {
    /// Create configuration from a ConfigLoader
    ///
    /// This is the primary way to create a RedisConfig using the config crate's
    /// configuration loading system. It supports loading from:
    /// - Shared .env file (libs/infrastructure/.env)
    /// - Service-specific .env file
    pub fn from_loader(loader: &ConfigLoader) -> ConfigResult<Self> {
        Ok(Self {
            url: loader.get_or("REDIS_URL", "redis://localhost:6379".to_string())?,
            key_prefix: loader.get_or("REDIS_KEY_PREFIX", "app".to_string())?,
            max_connections: loader.get_or("REDIS_MAX_CONNECTIONS", 50u32)?,
            connection_timeout: Duration::seconds(
                loader.get_or("REDIS_CONNECTION_TIMEOUT", 10i64)?,
            ),
            command_timeout: Duration::seconds(loader.get_or("REDIS_COMMAND_TIMEOUT", 5i64)?),
            retry_delay: Duration::milliseconds(loader.get_or("REDIS_RETRY_DELAY", 100i64)?),
            domains: RedisDomainsConfig {
                session: SessionConfig {
                    ttl: Duration::days(loader.get_or("SESSION_TTL_DAYS", 7i64)?),
                    max_sessions_per_user: loader.get_or("MAX_SESSIONS_PER_USER", 5u32)?,
                    refresh_threshold: loader.get_or("SESSION_REFRESH_THRESHOLD", 0.8f32)?,
                },
                rate_limit: RateLimitConfig {
                    window: Duration::minutes(loader.get_or("RATE_LIMIT_WINDOW_MINUTES", 15i64)?),
                    default_limit: loader.get_or("RATE_LIMIT_DEFAULT_LIMIT", 100u64)?,
                },
                cache: CacheConfig {
                    ttl: Duration::minutes(loader.get_or("CACHE_TTL_MINUTES", 10i64)?),
                },
            },
        })
    }

    /// Validate configuration invariants
    pub fn validate(&self) -> ConfigResult<()> {
        if self.url.trim().is_empty() {
            return Err(ConfigError::validation("REDIS_URL cannot be empty"));
        }

        if self.domains.session.refresh_threshold <= 0.0
            || self.domains.session.refresh_threshold > 1.0
        {
            return Err(ConfigError::validation(
                "SESSION_REFRESH_THRESHOLD must be between 0 and 1",
            ));
        }

        Ok(())
    }

    /// Build a fully namespaced Redis key
    pub fn key(&self, domain: &str, key: &str) -> String {
        // reuse RedisKey builder to ensure formatting stays in one place
        crate::redis::key::RedisKey::from_parts([
            &self.key_prefix,
            domain,
            key,
        ])
        .into()
    }

    pub fn parsed_url(&self) -> ConfigResult<Url> {
        Url::parse(&self.url).map_err(|_| ConfigError::validation("Invalid REDIS_URL"))
    }

    /// Extract host from Redis URL (for diagnostics/logging)
    pub fn host(&self) -> Option<String> {
        self.parsed_url().ok()?.host_str().map(|s| s.to_string())
    }

    pub fn port(&self) -> Option<u16> {
        self.parsed_url().ok()?.port()
    }

    pub fn is_tls(&self) -> bool {
        self.parsed_url()
            .ok()
            .map(|u| u.scheme() == "rediss")
            .unwrap_or(false)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_redis_config_defaults() {
//         let config = RedisConfig::default();
//         assert!(!config.url.is_empty());
//         assert!(!config.key_prefix.is_empty());
//     }
//
//     #[test]
//     fn test_redis_config_url_parsing() {
//         let config = RedisConfig {
//             url: "redis://localhost:6379".to_string(),
//             ..Default::default()
//         };
//
//         assert_eq!(config.host(), Some("localhost"));
//         assert_eq!(config.port(), Some(6379));
//     }
//
//     #[test]
//     fn test_redis_config_validation() {
//         let mut config = RedisConfig::default();
//         assert!(config.validate().is_ok());
//
//         config.url = "".to_string();
//         assert!(config.validate().is_err());
//     }
// }
