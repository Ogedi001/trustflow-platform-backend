//! Configuration loader
//!
//! Provides a unified configuration loader that combines multiple sources.

use crate::core::{
    environment::Environment,
    error::{ConfigError, ConfigResult},
};
use crate::sources::{dotenv::DotenvSource, yaml::YamlSource};
#[derive(Debug, Clone)]
pub enum ConfigSource {
    Dotenv(DotenvSource),
    Yaml(YamlSource),
}

#[derive(Debug, Clone, Default)]
pub struct ConfigLoader {
    sources: Vec<ConfigSource>,
}

impl ConfigLoader {
    pub fn new() -> Self {
        Self { sources: vec![] }
    }

    /// Load shared libs/config/.env
    pub fn with_shared_env(mut self, source: DotenvSource) -> Self {
        self.sources.push(ConfigSource::Dotenv(source));
        self
    }

    /// Load service-specific .env
    pub fn with_service_env(mut self, source: DotenvSource) -> Self {
        self.sources.push(ConfigSource::Dotenv(source));
        self
    }

    /// Add YAML source
    pub fn with_yaml(mut self, source: YamlSource) -> Self {
        self.sources.push(ConfigSource::Yaml(source));
        self
    }

    /// Get the application environment (APP_ENV) from the sources, defaulting to development
    pub fn environment(&self) -> Environment {
        self.get_or("APP_ENV", "development".to_string())
            .and_then(|v| {
                v.parse::<Environment>()
                    .map_err(|e| ConfigError::parse("APP_ENV", e))
            })
            .unwrap_or_default()
    }

    pub fn get<T>(&self, key: &str) -> ConfigResult<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        self.get_inner(key, true)
    }

    pub fn get_or<T>(&self, key: &str, default: T) -> ConfigResult<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        match self.get_inner::<T>(key, false) {
            Ok(v) => Ok(v),
            Err(ConfigError::Missing { .. }) => Ok(default),
            Err(e) => Err(e),
        }
    }

    fn get_inner<T>(&self, key: &str, required: bool) -> ConfigResult<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        // iterate in reverse so last-added source wins
        for source in self.sources.iter().rev() {
            match source {
                ConfigSource::Dotenv(dotenv) => {
                    if let Some(value) = dotenv.get(key) {
                        return value
                            .parse::<T>()
                            .map_err(|e| ConfigError::parse(key, e.to_string()));
                    }
                }

                ConfigSource::Yaml(yaml) => {
                    if let Some(value) = yaml.get(key) {
                        let s = match value {
                            serde_json::Value::String(v) => v.clone(),
                            serde_json::Value::Number(v) => v.to_string(),
                            serde_json::Value::Bool(v) => v.to_string(),
                            _ => serde_json::to_string(value)
                                .map_err(|e| ConfigError::parse(key, e.to_string()))?,
                        };

                        return s
                            .parse::<T>()
                            .map_err(|e| ConfigError::parse(key, e.to_string()));
                    }
                }
            }
        }

        if required {
            Err(ConfigError::missing(key))
        } else {
            Err(ConfigError::missing(key))
        }
    }

    pub fn contains(&self, key: &str) -> bool {
        self.sources.iter().any(|source| match source {
            ConfigSource::Dotenv(d) => d.contains(key),
            ConfigSource::Yaml(y) => y.contains(key),
        })
    }
}
