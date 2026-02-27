//! Infrastructure module for Identity Service
//!
//! Database connections, repositories, and external service integrations.
//! Uses shared infrastructure library for Redis and Database utilities.

pub mod db;
pub mod repositories;

use common::Timestamp;
use infrastructure::database::{DbPool, DbPoolConfig};
use infrastructure::redis::{RedisConfig, RedisPool};
use std::sync::Arc;

/// Infrastructure context - shared by all services
#[derive(Clone)]
pub struct Infrastructure {
    pub db: DbPool,
    pub redis: RedisPool,
    pub config: InfrastructureConfig,
}

/// Infrastructure configuration
#[derive(Clone, Debug)]
pub struct InfrastructureConfig {
    pub db: DbPoolConfig,
    pub redis: RedisConfig,
}

impl Default for InfrastructureConfig {
    fn default() -> Self {
        Self {
            db: DbPoolConfig::default(),
            redis: RedisConfig::default(),
        }
    }
}

impl Infrastructure {
    /// Create new infrastructure from config
    pub async fn new(
        config: InfrastructureConfig,
    ) -> Result<Self, infrastructure::database::DbPoolError> {
        // Create database pool using shared infrastructure
        let db = DbPool::new(&config.db).await?;

        // Create Redis pool using infrastructure library
        let redis = RedisPool::new(&config.redis.url)
            .await
            .map_err(|e| infrastructure::database::DbPoolError::Configuration(e.to_string()))?;

        Ok(Self { db, redis, config })
    }

    /// Get database pool
    pub fn db(&self) -> &DbPool {
        &self.db
    }

    /// Get Redis pool
    pub fn redis(&self) -> &RedisPool {
        &self.redis
    }
}

/// Unit of work for transaction management
#[derive(Clone)]
pub struct UnitOfWork<'a> {
    db: &'a DbPool,
    // Add other resources as needed
}

impl<'a> UnitOfWork<'a> {
    pub fn new(db: &'a DbPool) -> Self {
        Self { db }
    }

    /// Begin a new transaction
    pub async fn begin(
        &self,
    ) -> Result<infrastructure::database::Transaction<'a>, infrastructure::database::DbPoolError>
    {
        self.db
            .begin()
            .await
            .map(infrastructure::database::Transaction::new)
    }
}

/// Repository context - provides access to repositories
#[derive(Clone)]
pub struct RepositoryContext<'a> {
    pub uow: UnitOfWork<'a>,
}

impl<'a> RepositoryContext<'a> {
    pub fn new(db: &'a PgPool) -> Self {
        Self {
            uow: UnitOfWork::new(db),
        }
    }
}
