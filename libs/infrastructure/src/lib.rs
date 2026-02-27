//! Infrastructure - Production-grade infrastructure components for microservices
//!
//! This library provides common infrastructure utilities that can be shared across all
//! microservices in the TrustFlow platform.
//!
//! ## Features
//!
//! - `database`: PostgreSQL connection pool and transaction management  
//! - `redis`: Redis client pool, session storage, rate limiting, caching, OTP, and distributed locks
//! - `resilience`: Circuit breaker, retry, timeout, and bulkhead patterns
//! - `discovery`: Service discovery with Consul support
//! - `observability`: Logging, metrics, and tracing
//! - `full`: Enables all features
//!
//! ## Usage
//!
//! ```ignore
//! use infrastructure::database::{DbPool, DatabaseConfig};
//! use infrastructure::redis::{RedisPool, RedisConfig};
//! use config::Settings;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Load config using config crate's Settings trait
//!     let config = DatabaseConfig::from_loader(loader)?;
//!     
//!     // Create database pool
//!     let pool = DbPool::new(&config).await?;
//!
//!     // Create Redis pool
//!     let redis_pool = RedisPool::new("redis://localhost:6379").await?;
//!
//!     Ok(())
//! }
//! ```

// Re-export error types from the error crate
pub use error::{AppError, AppResult};

// ============================================================================
// Module Declarations
// ============================================================================

// Core modules - always available
pub mod config;
pub mod discovery;
pub mod resilience;

// Feature-gated modules
#[cfg(feature = "database")]
pub mod database;

#[cfg(feature = "redis")]
pub mod redis;

#[cfg(feature = "observability")]
pub mod observability;

#[cfg(feature = "http")]
pub mod http_clients;

#[cfg(feature = "scheduler")]
pub mod scheduler;

#[cfg(feature = "search")]
pub mod search;

#[cfg(feature = "messaging")]
pub mod messaging;

#[cfg(feature = "sms")]
pub mod sms;

#[cfg(feature = "storage")]
pub mod storage;

// ============================================================================
// Conditional Exports
// ============================================================================

// Database exports
#[cfg(feature = "database")]
pub use database::{
    config::DatabaseConfig,
    health::HealthChecker,
    migrations_dir,
    pool::{DbPool, DbPoolConfig, DbPoolError},
    repository::{Repository, RepositoryExt},
    run_migrations,
    transaction::Transaction,
};

// Redis exports
#[cfg(feature = "redis")]
pub use redis::{
    cache::{Cache, RedisCache},
    config::RedisConfig,
    error::RedisError,
    lock::DistributedLock,
    otp::OtpCache,
    pool::RedisPool,
    pubsub::{PubSub, PubSubMessage, RedisPubSub},
    rate_limiter::{RateLimiter, RedisRateLimiter},
    session::{RedisSessionStore, SessionData, SessionStore},
};

// Resilience exports
pub use resilience::{
    bulkhead::{Bulkhead, BulkheadConfig},
    circuit_breaker::{CircuitBreaker, CircuitBreakerConfig, CircuitBreakerState},
    retry::{ExponentialBackoff, RetryConfig, RetryPolicy},
    timeout::TimeoutError,
};

// Discovery exports
#[cfg(feature = "discovery")]
pub use discovery::{
    ServiceDiscovery,
    consul::{ConsulClient, ConsulConfig},
};

// Config exports (Settings trait implementations)
pub use config::{database::DatabaseSettings, redis::RedisSettings};
