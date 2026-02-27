//! Shared infrastructure layer for the modular monolith.
//!
//! This crate intentionally provides a small, stable surface:
//! - `database`: one PostgreSQL pool configuration and builder
//! - `redis`: one Redis connection manager configuration and builder
//! - `config`: thin re-exports of shared configuration loader utilities

pub use error::{AppError, AppResult};

pub mod config;

#[cfg(feature = "database")]
pub mod database;

#[cfg(feature = "redis")]
pub mod redis;

#[cfg(feature = "database")]
pub use database::{DatabaseConfig, DbPool, DbPoolError};

#[cfg(feature = "redis")]
pub use redis::{RedisConfig, RedisError, RedisPool};
