//! PostgreSQL pool wrapper.

use std::time::Duration as StdDuration;

use sqlx::{PgPool, postgres::PgPoolOptions};
use thiserror::Error;

use crate::database::config::DatabaseConfig;

#[derive(Clone)]
pub struct DbPool {
    pool: PgPool,
}

impl DbPool {
    pub async fn new(config: &DatabaseConfig) -> Result<Self, DbPoolError> {
        config.validate()?;

        let pool_cfg = &config.pool;
        let pool = PgPoolOptions::new()
            .max_connections(pool_cfg.max_connections)
            .min_connections(pool_cfg.min_connections)
            .acquire_timeout(to_std_duration(pool_cfg.acquire_timeout))
            .idle_timeout(Some(to_std_duration(pool_cfg.idle_timeout)))
            .max_lifetime(Some(to_std_duration(pool_cfg.max_lifetime)))
            .connect(&config.url)
            .await?;

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn close(&self) {
        self.pool.close().await;
    }
}

fn to_std_duration(duration: time::Duration) -> StdDuration {
    let secs = duration.whole_seconds().max(0) as u64;
    StdDuration::from_secs(secs)
}

#[derive(Debug, Error)]
pub enum DbPoolError {
    #[error("database configuration error: {0}")]
    Configuration(String),

    #[error("database connection error: {0}")]
    Connection(#[from] sqlx::Error),
}

impl From<config::core::error::ConfigError> for DbPoolError {
    fn from(value: config::core::error::ConfigError) -> Self {
        Self::Configuration(value.to_string())
    }
}
