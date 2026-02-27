//! Shared configuration exports for infrastructure wiring.

pub use config::core::error::{ConfigError, ConfigResult};
pub use config::loader::ConfigLoader;
pub use config::sources::dotenv::DotenvSource;

#[cfg(feature = "database")]
pub use crate::database::DatabaseConfig;

#[cfg(feature = "redis")]
pub use crate::redis::RedisConfig;
