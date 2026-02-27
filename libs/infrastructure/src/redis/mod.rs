//! Redis infrastructure for all services
//!
//! Provides shared Redis client pool, session storage, rate limiting, and caching utilities.
//!
//! ## Feature Flags
//!
//! - `redis`: Enables Redis support (enabled by default with `full` feature)

pub mod cache;
pub mod config;
pub mod key;
pub mod lock;
pub mod otp;
pub mod pool;
pub mod pubsub;
pub mod rate_limiter;
pub mod session;

pub mod error;

// Public exports - re-export from submodules
#[cfg(feature = "redis")]
pub use cache::{Cache, RedisCache};
#[cfg(feature = "redis")]
pub use config::RedisConfig;
#[cfg(feature = "redis")]
pub use error::RedisError;
#[cfg(feature = "redis")]
pub use key::RedisKey;
#[cfg(feature = "redis")]
pub use lock::DistributedLock;
#[cfg(feature = "redis")]
pub use otp::OtpCache;
#[cfg(feature = "redis")]
pub use pool::RedisPool;
#[cfg(feature = "redis")]
pub use pubsub::PubSub;
#[cfg(feature = "redis")]
pub use rate_limiter::{RateLimiter, RedisRateLimiter};
#[cfg(feature = "redis")]
pub use session::{RedisSessionStore, SessionData, SessionStore};
