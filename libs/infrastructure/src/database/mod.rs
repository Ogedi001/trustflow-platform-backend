//! Database infrastructure for all services
//!
//! Provides database connection pool, migrations, health checks, and utilities using SQLx.
//!
//! ## Feature Flags
//!
//! - `database`: Enables database support (enabled by default)
//!
//! ## Features
//!
//! - Connection pooling with configurable limits
//! - Transaction management
//! - Database migrations
//! - Health checks for monitoring
//! - Comprehensive error handling

#[cfg(feature = "database")]
pub mod pool;

#[cfg(feature = "database")]
pub mod transaction;

#[cfg(feature = "database")]
pub mod health;

#[cfg(feature = "database")]
pub mod config;

#[cfg(feature = "database")]
pub mod repository;

#[cfg(feature = "database")]
pub use pool::{DbPool, DbPoolConfig, DbPoolError};

#[cfg(feature = "database")]
pub use transaction::Transaction;

#[cfg(feature = "database")]
pub use health::HealthChecker;

#[cfg(feature = "database")]
use sqlx::{Postgres, migrate::Migrator};
#[cfg(feature = "database")]
use std::path::Path;
#[cfg(feature = "database")]
use tracing::info;

/// Run database migrations
#[cfg(feature = "database")]
pub async fn run_migrations(
    pool: &DbPool,
    migrations_path: &Path,
) -> Result<(), sqlx::migrate::MigrateError> {
    info!("Running database migrations from {:?}", migrations_path);

    let m = Migrator::new(migrations_path).await?;
    m.run(pool.pool()).await?;

    info!("Migrations completed successfully");
    Ok(())
}

/// Create migrations directory structure
#[cfg(feature = "database")]
pub fn migrations_dir() -> std::io::Result<std::path::PathBuf> {
    let mut path = std::env::current_dir()?;
    path.push("services");
    path.push("identity");
    path.push("migrations");
    Ok(path)
}
