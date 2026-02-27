//! Application configuration for Identity Service
//!
//! Loads configuration from environment variables and provides typed access.
//! Uses the centralized trustflow_config library.

use config::identity::{MfaConfig, PasswordConfig, RateLimitConfig, VerificationConfig};
use config::{
    DatabaseConfig, Environment as ConfigEnvironment, JwtConfig, RedisConfig, ServerConfig,
};
use serde::Deserialize;
use std::net::SocketAddr;

/// Main application configuration
#[derive(Clone, Debug)]
pub struct Config {
    pub server: ServerConfig,
    pub environment: Environment,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub jwt: JwtConfig,
    pub mfa: MfaConfig,
    pub rate_limit: RateLimitConfig,
    pub verification: VerificationConfig,
    pub password: PasswordConfig,
}

/// Infrastructure configuration (for database and redis connections)
#[derive(Clone, Debug)]
pub struct InfrastructureConfig {
    pub db: DatabaseConfig,
    pub redis: RedisConfig,
}

/// Environment type
#[derive(Clone, Debug, Deserialize)]
pub enum Environment {
    Development,
    Testing,
    Staging,
    Production,
}

impl Default for Environment {
    fn default() -> Self {
        Self::Development
    }
}

impl Environment {
    pub fn is_production(&self) -> bool {
        matches!(self, Self::Production)
    }
}

impl From<ConfigEnvironment> for Environment {
    fn from(env: ConfigEnvironment) -> Self {
        match env {
            ConfigEnvironment::Development => Environment::Development,
            ConfigEnvironment::Testing => Environment::Testing,
            ConfigEnvironment::Staging => Environment::Staging,
            ConfigEnvironment::Production => Environment::Production,
        }
    }
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        Self {
            server: ServerConfig::from_env(),
            environment: std::env::var("ENVIRONMENT")
                .unwrap_or_else(|_| "development".to_string())
                .parse()
                .unwrap_or(Environment::Development),
            database: DatabaseConfig::from_env(),
            redis: RedisConfig::from_env(),
            jwt: JwtConfig::from_env(),
            mfa: MfaConfig::from_env(),
            rate_limit: RateLimitConfig::from_env(),
            verification: VerificationConfig::from_env(),
            password: PasswordConfig::from_env(),
        }
    }
}

impl InfrastructureConfig {
    pub fn from_env() -> Self {
        Self {
            db: DatabaseConfig::from_env(),
            redis: RedisConfig::from_env(),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_config_defaults() {
//         std::env::remove_var("DATABASE_URL");
//         std::env::remove_var("REDIS_URL");
//         std::env::remove_var("JWT_SECRET");

//         let config = Config::from_env();

//         assert_eq!(config.server_address, "0.0.0.0:8080");
//         assert_eq!(config.database.url, "postgres://localhost:5432/identity");
//         assert_eq!(config.redis.url, "redis://localhost:6379");
//     }

//     #[test]
//     fn test_environment_parsing() {
//         assert_eq!(
//             "development".parse::<Environment>(),
//             Ok(Environment::Development)
//         );
//         assert_eq!(
//             "production".parse::<Environment>(),
//             Ok(Environment::Production)
//         );
//         assert!("invalid".parse::<Environment>().is_err());
//     }
// }
