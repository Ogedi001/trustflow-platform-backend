//! Configuration sources module
//!
//! Provides different configuration sources for loading settings:
//! - Environment variables (via dotenvy for safe .env file loading)
//! - YAML files
//! - JSON files
//! - Custom sources

pub mod dotenv;
pub mod yaml;
