//! Database infrastructure shared by all domains.

pub mod config;
pub mod pool;

pub use config::DatabaseConfig;
pub use pool::{DbPool, DbPoolError};

use std::path::Path;

use sqlx::migrate::Migrator;

/// Run SQLx migrations from a given directory.
pub async fn run_migrations(
    pool: &DbPool,
    migrations_path: &Path,
) -> Result<(), sqlx::migrate::MigrateError> {
    let migrator = Migrator::new(migrations_path).await?;
    migrator.run(pool.pool()).await
}
