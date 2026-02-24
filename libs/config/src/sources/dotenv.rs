//! Dotenv configuration source using dotenvy
//!
//! Provides explicit, deterministic .env file loading using dotenvy.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::core::error::{ConfigError, ConfigResult};

/// A configuration source that loads from .env files using dotenvy
#[derive(Debug, Clone)]
pub struct DotenvSource {
    vars: HashMap<String, String>,
    loaded_files: Vec<PathBuf>,
}

impl DotenvSource {
    /// Create a new empty dotenv source
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            loaded_files: Vec::new(),
        }
    }
    /// Load from a specific .env file
    pub fn from_file(path: impl AsRef<Path>) -> ConfigResult<Self> {
        let path = path.as_ref();

        if !path.exists() {
            return Err(ConfigError::missing(format!(
                "Env file not found: {}",
                path.display()
            )));
        }

        let vars = dotenvy::from_path_iter(path)
            .map_err(|e| ConfigError::parse(path.to_string_lossy(), e.to_string()))?
            .collect::<Result<HashMap<String, String>, dotenvy::Error>>()
            .map_err(|e| ConfigError::parse(path.to_string_lossy(), e.to_string()))?;

        Ok(Self {
            vars,
            loaded_files: vec![path.to_path_buf()],
        })
    }

    /// Try load file, return empty source if missing
    pub fn try_from_file(path: impl AsRef<Path>) -> ConfigResult<Self> {
        let path = path.as_ref();

        if !path.exists() {
            return Ok(Self::new());
        }

        Self::from_file(path)
    }

    /// Get value
    pub fn get(&self, key: &str) -> Option<&str> {
        self.vars.get(key).map(|v| v.as_str())
    }

    /// Check existence
    pub fn contains(&self, key: &str) -> bool {
        self.vars.contains_key(key)
    }

    /// Iterate values
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.vars.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }

    /// Loaded file paths
    pub fn loaded_files(&self) -> &[PathBuf] {
        &self.loaded_files
    }

    /// Merge another source (other overrides self)
    pub fn merge(mut self, other: DotenvSource) -> Self {
        self.vars.extend(other.vars);
        self.loaded_files.extend(other.loaded_files);
        self
    }

    /// Convert into HashMap
    pub fn into_map(self) -> HashMap<String, String> {
        self.vars
    }
}

impl Default for DotenvSource {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for layered dotenv loading
#[derive(Debug, Default)]
pub struct DotenvLayerBuilder {
    layers: Vec<DotenvSource>,
}

impl DotenvLayerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Lowest priority (shared libs/config/.env)
    pub fn with_shared(mut self, path: impl AsRef<Path>) -> ConfigResult<Self> {
        self.layers.push(DotenvSource::try_from_file(path)?);
        Ok(self)
    }

    /// Service-specific .env
    pub fn with_service(mut self, path: impl AsRef<Path>) -> ConfigResult<Self> {
        self.layers.push(DotenvSource::try_from_file(path)?);
        Ok(self)
    }

    /// Local developer overrides
    pub fn with_local(mut self, path: impl AsRef<Path>) -> ConfigResult<Self> {
        self.layers.push(DotenvSource::try_from_file(path)?);
        Ok(self)
    }

    /// Explicit override
    pub fn with_override(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let mut source = DotenvSource::new();
        source.vars.insert(key.into(), value.into());
        self.layers.push(source);
        self
    }

    /// Build merged source
    pub fn build(self) -> DotenvSource {
        self.layers
            .into_iter()
            .fold(DotenvSource::new(), |acc, layer| acc.merge(layer))
    }
}
