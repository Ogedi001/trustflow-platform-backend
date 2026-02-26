//! Database configuration
//!
//! Centralized database configuration for the modular monolith.
//! All domains share a single database instance with domain-specific
//! behavior configured through sub-structures (pool, logging, migrations).
//!
//! This avoids premature microservice complexity while remaining
//! future-ready for service extraction.

// Re-export config types for convenience
pub use config::core::environment::Environment;
pub use config::core::error::{ConfigError, ConfigResult};
pub use config::loader::ConfigLoader;
use serde::{Deserialize, Serialize};
use time::Duration;
use url::Url;

/// Top-level database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database connection URL
    pub url: String,

    /// Connection pool configuration
    pub pool: PoolConfig,

    /// Logging configuration
    pub logging: LoggingConfig,

    /// Migration configuration
    pub migrations: MigrationConfig,
}

/// Connection pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolConfig {
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_lifetime: Duration,
}

/// Database logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub statement_logging: bool,
    pub connect_logging: bool,
}

/// Database migration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationConfig {
    /// Path to migration files
    pub path: String,

    /// Whether to auto-run migrations on startup
    pub auto_migrate: bool,

    /// Whether to clean placeholder tables before migration
    pub clean_before_migrate: bool,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "postgres://postgres:password@localhost:5432/trustflow".to_string(),
            pool: PoolConfig::default(),
            logging: LoggingConfig::default(),
            migrations: MigrationConfig::default(),
        }
    }
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_connections: 50,
            min_connections: 5,
            acquire_timeout: Duration::seconds(30),
            idle_timeout: Duration::seconds(600),
            max_lifetime: Duration::seconds(1800),
        }
    }
}
impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            statement_logging: false,
            connect_logging: false,
        }
    }
}

impl Default for MigrationConfig {
    fn default() -> Self {
        Self {
            path: "migrations".to_string(),
            auto_migrate: false,
            clean_before_migrate: false,
        }
    }
}

impl DatabaseConfig {
    /// Load database configuration from ConfigLoader
    pub fn from_loader(loader: &ConfigLoader) -> ConfigResult<Self> {
        Ok(Self {
            url: loader.get_or(
                "DATABASE_URL",
                "postgres://postgres:password@localhost:5432/trustflow".to_string(),
            )?,
            pool: PoolConfig {
                max_connections: loader.get_or("DATABASE_MAX_CONNECTIONS", 50u32)?,
                min_connections: loader.get_or("DATABASE_MIN_CONNECTIONS", 5u32)?,
                acquire_timeout: Duration::seconds(
                    loader.get_or("DATABASE_ACQUIRE_TIMEOUT", 30i64)?,
                ),
                idle_timeout: Duration::seconds(loader.get_or("DATABASE_IDLE_TIMEOUT", 600i64)?),
                max_lifetime: Duration::seconds(loader.get_or("DATABASE_MAX_LIFETIME", 1800i64)?),
            },
            logging: LoggingConfig {
                statement_logging: loader.get_or("DATABASE_STATEMENT_LOGGING", false)?,
                connect_logging: loader.get_or("DATABASE_CONNECT_LOGGING", false)?,
            },
            migrations: MigrationConfig {
                path: loader.get_or("MIGRATIONS_PATH", "migrations".to_string())?,
                auto_migrate: loader.get_or("AUTO_MIGRATE", false)?,
                clean_before_migrate: loader.get_or("CLEAN_BEFORE_MIGRATE", false)?,
            },
        })
    }

    /// Validate configuration invariants
    pub fn validate(&self) -> ConfigResult<()> {
        if self.url.trim().is_empty() {
            return Err(ConfigError::validation("DATABASE_URL cannot be empty"));
        }

        if self.pool.max_connections < self.pool.min_connections {
            return Err(ConfigError::validation(
                "DATABASE_MAX_CONNECTIONS must be >= DATABASE_MIN_CONNECTIONS",
            ));
        }

        Ok(())
    }

    /// Parsed database URL (safe, structured)
    pub fn parsed_url(&self) -> ConfigResult<Url> {
        Url::parse(&self.url).map_err(|_| ConfigError::validation("Invalid DATABASE_URL"))
    }

    /// Database host (for diagnostics/logging)
    pub fn host(&self) -> Option<String> {
        self.parsed_url().ok()?.host_str().map(|s| s.to_string())
    }

    /// Database port (for diagnostics/logging)
    pub fn port(&self) -> Option<u16> {
        self.parsed_url().ok()?.port()
    }

    /// Database name (for diagnostics/logging)
    pub fn database_name(&self) -> Option<String> {
        self.parsed_url()
            .ok()?
            .path_segments()?
            .last()
            .map(|s| s.to_string())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_database_config_defaults() {
//         let config = DatabaseConfig::default();
//         assert!(!config.url.is_empty());
//         assert!(config.max_connections >= config.min_connections);
//     }

//     #[test]
//     fn test_database_config_url_parsing() {
//         let config = DatabaseConfig {
//             url: "postgres://user:pass@localhost:5432/mydb".to_string(),
//             ..Default::default()
//         };

//         assert_eq!(config.host(), Some("localhost"));
//         assert_eq!(config.port(), Some(5432));
//         assert_eq!(config.database_name(), Some("mydb"));
//     }

//     #[test]
//     fn test_database_config_validation() {
//         let mut config = DatabaseConfig::default();
//         assert!(config.validate(&Environment::Development).is_ok());

//         config.url = "".to_string();
//         assert!(config.validate(&Environment::Development).is_err());

//         config.url = "postgres://localhost:5432/db".to_string();
//         config.max_connections = 5;
//         config.min_connections = 10;
//         assert!(config.validate(&Environment::Development).is_err());
//     }
// }
