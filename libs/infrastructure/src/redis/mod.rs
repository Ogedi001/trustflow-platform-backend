//! Redis infrastructure shared by all domains.

pub mod config;
pub mod error;
pub mod pool;

pub use config::RedisConfig;
pub use error::RedisError;
pub use pool::RedisPool;
