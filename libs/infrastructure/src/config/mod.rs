//! Infrastructure configuration module
//!
//! This module provides a slim interface for infrastructure configuration.
//! The actual config types are defined in:
//! - `database::config::DatabaseConfig` - Database configuration
//! - `redis::config::RedisConfig` - Redis configuration

pub mod database;
pub mod redis;
pub use config::core::error::{ConfigError, ConfigResult};
pub use config::loader::ConfigLoader;
pub use config::sources::dotenv::DotenvSource;
pub use database::DatabaseConfig;
pub use redis::RedisConfig;
