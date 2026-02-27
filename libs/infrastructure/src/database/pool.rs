//! Database connection pool utilities
//!
//! Provides connection pool management for PostgreSQL using SQLx.
//!
//! ## Feature Flags
//!
//! - `database`: Enables database support (enabled by default with `full` feature)

use crate::database::config::DatabaseConfig;
#[cfg(feature = "database")]
use sqlx::{pool::PoolOptions, postgres::PgPool, postgres::PgPoolOptions};
#[cfg(feature = "database")]
use thiserror::Error;
#[cfg(feature = "database")]
use tracing::info;

// Re-export DatabaseConfig from config module for convenience
pub use crate::database::config::DatabaseConfig as DbPoolConfig;

/// Database connection pool
#[cfg(feature = "database")]
#[derive(Clone)]
pub struct DbPool {
    pool: PgPool,
}

#[cfg(feature = "database")]
impl DbPool {
    /// Create a new database pool from DatabaseConfig
    ///
    /// Uses the DatabaseConfig from database/config.rs which uses Duration
    /// from the time crate for better type safety.
    pub async fn new(config: &DatabaseConfig) -> Result<Self, DbPoolError> {
        info!("Connecting to database at {}", config.url);

        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(config.acquire_timeout)
            .idle_timeout(config.idle_timeout)
            .max_lifetime(config.max_lifetime)
            .connect(&config.url)
            .await?;

        info!(
            "Database connected successfully. Pool size: {}",
            config.max_connections
        );

        Ok(Self { pool })
    }

    /// Get the underlying pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Get mutable pool reference
    pub fn pool_mut(&mut self) -> &mut PgPool {
        &mut self.pool
    }

    /// Acquire a connection from the pool
    pub async fn acquire(
        &self,
    ) -> Result<sqlx::pool::PoolConnection<'_, sqlx::Postgres>, DbPoolError> {
        Ok(self.pool.acquire().await?)
    }

    /// Begin a new transaction
    pub async fn begin(&self) -> Result<sqlx::Transaction<'_, sqlx::Postgres>, DbPoolError> {
        Ok(self.pool.begin().await?)
    }

    /// Execute a query
    pub async fn execute(&self, query: &str) -> Result<sqlx::postgres::PgQueryResult, DbPoolError> {
        sqlx::query(query)
            .execute(&self.pool)
            .await
            .map_err(|e| e.into())
    }

    /// Execute a query with parameters
    pub async fn execute_typed<T>(
        &self,
        query: &sqlx::query::Query<'_, T, sqlx::postgres::PgArguments>,
    ) -> Result<sqlx::postgres::PgQueryResult, DbPoolError> {
        query.execute(&self.pool).await.map_err(|e| e.into())
    }

    /// Fetch one row
    pub async fn fetch_one<T>(&self, query: &str) -> Result<T, DbPoolError>
    where
        T: sqlx::FromRow<sqlx::postgres::PgRow>,
    {
        sqlx::query_as::<_, T>(query)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.into())
    }

    /// Fetch all rows
    pub async fn fetch_all<T>(&self, query: &str) -> Result<Vec<T>, DbPoolError>
    where
        T: sqlx::FromRow<sqlx::postgres::PgRow>,
    {
        sqlx::query_as::<_, T>(query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.into())
    }

    /// Fetch optional row
    pub async fn fetch_optional<T>(&self, query: &str) -> Result<Option<T>, DbPoolError>
    where
        T: sqlx::FromRow<sqlx::postgres::PgRow>,
    {
        sqlx::query_as::<_, T>(query)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.into())
    }

    /// Close the pool
    pub async fn close(&self) {
        self.pool.close().await
    }
}

/// Database pool errors
#[cfg(feature = "database")]
#[derive(Debug, Error)]
pub enum DbPoolError {
    #[error("Connection error: {0}")]
    Connection(#[from] sqlx::Error),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Transaction error: {0}")]
    Transaction(#[from] sqlx::Error),
}

#[cfg(feature = "database")]
impl From<sqlx::Error> for DbPoolError {
    fn from(e: sqlx::Error) -> Self {
        Self::Connection(e)
    }
}

#[cfg(feature = "database")]
impl From<String> for DbPoolError {
    fn from(e: String) -> Self {
        Self::Configuration(e)
    }
}

/// Convert from sqlx::Error to AppError
#[cfg(feature = "database")]
impl From<sqlx::Error> for error::AppError {
    fn from(e: sqlx::Error) -> Self {
        error::AppError::database(e.to_string())
    }
}

/// Convert from DbPoolError to AppError
#[cfg(feature = "database")]
impl From<DbPoolError> for error::AppError {
    fn from(e: DbPoolError) -> Self {
        match e {
            DbPoolError::Connection(err) => error::AppError::database(err.to_string()),
            DbPoolError::Configuration(msg) => error::AppError::validation(msg),
            DbPoolError::Transaction(err) => error::AppError::database(err.to_string()),
        }
    }
}
