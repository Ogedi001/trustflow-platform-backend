//! YAML configuration source
//!
//! Deterministic YAML loader with dot-notation lookup and merge support.
//! Does NOT read OS environment variables implicitly.

use std::collections::HashMap;
use std::path::Path;

use crate::core::error::{ConfigError, ConfigResult};

#[derive(Debug, Clone)]
pub struct YamlSource {
    name: String,
    values: serde_json::Value,
}

impl YamlSource {
    /// Load YAML from file
    pub fn from_file(path: impl AsRef<Path>) -> ConfigResult<Self> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)
            .map_err(|e| ConfigError::file_read(path.display().to_string(), e))?;

        Self::from_str(path.display().to_string(), &content)
    }

    /// Load YAML from string
    pub fn from_str(name: impl Into<String>, content: &str) -> ConfigResult<Self> {
        let values: serde_json::Value = serde_yaml::from_str(content)
            .map_err(|e| ConfigError::source(format!("YAML parse error: {e}")))?;

        Ok(Self {
            name: name.into(),
            values,
        })
    }

    /// Load YAML with variable interpolation (from provided map, NOT OS)
    pub fn from_file_with_vars(
        path: impl AsRef<Path>,
        vars: &HashMap<String, String>,
    ) -> ConfigResult<Self> {
        let path = path.as_ref();
        let raw = std::fs::read_to_string(path)
            .map_err(|e| ConfigError::file_read(path.display().to_string(), e))?;

        let interpolated = interpolate_vars(&raw, vars)?;
        Self::from_str(path.display().to_string(), &interpolated)
    }

    /// Get raw JSON value using dot-notation
    pub fn get(&self, key: &str) -> Option<&serde_json::Value> {
        let mut current = &self.values;

        for part in key.split('.') {
            current = match current {
                serde_json::Value::Object(map) => map.get(part)?,
                serde_json::Value::Array(arr) => {
                    let idx = part.parse::<usize>().ok()?;
                    arr.get(idx)?
                }
                _ => return None,
            };
        }

        Some(current)
    }

    /// Get required typed value
    pub fn get_required<T>(&self, key: &str) -> ConfigResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let value = self.get(key).ok_or_else(|| ConfigError::missing(key))?;

        serde_json::from_value(value.clone()).map_err(|e| ConfigError::parse(key, e.to_string()))
    }

    /// Get optional typed value
    pub fn get_or<T>(&self, key: &str, default: T) -> ConfigResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        match self.get(key) {
            Some(v) => serde_json::from_value(v.clone())
                .map_err(|e| ConfigError::parse(key, e.to_string())),
            None => Ok(default),
        }
    }

    /// Deserialize entire YAML into struct
    pub fn deserialize<T>(&self) -> ConfigResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        serde_json::from_value(self.values.clone())
            .map_err(|e| ConfigError::source(format!("Deserialize error: {e}")))
    }

    pub fn contains(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &serde_json::Value {
        &self.values
    }
}

/* ===================== MERGING ===================== */

pub fn merge_sources(sources: &[YamlSource]) -> serde_json::Value {
    let mut result = serde_json::Value::Object(serde_json::Map::new());

    for source in sources {
        merge_values(&mut result, source.value());
    }

    result
}

fn merge_values(target: &mut serde_json::Value, source: &serde_json::Value) {
    match (target, source) {
        (serde_json::Value::Object(target_map), serde_json::Value::Object(source_map)) => {
            for (key, value) in source_map {
                match target_map.get_mut(key) {
                    Some(existing) => merge_values(existing, value),
                    None => {
                        target_map.insert(key.clone(), value.clone());
                    }
                }
            }
        }
        (target, source) => {
            *target = source.clone();
        }
    }
}

/* ===================== INTERPOLATION ===================== */

/// Replace ${VAR} or ${VAR:-default} from provided map
fn interpolate_vars(content: &str, vars: &HashMap<String, String>) -> ConfigResult<String> {
    let regex = regex::Regex::new(r"\$\{([^}:]+)(?::-(.*?))?\}")
        .map_err(|e| ConfigError::source(format!("Regex error: {e}")))?;

    let result = regex.replace_all(content, |caps: &regex::Captures| {
        let key = &caps[1];
        let default = caps.get(2).map(|m| m.as_str());

        vars.get(key)
            .cloned()
            .or_else(|| default.map(|d| d.to_string()))
            .unwrap_or_default()
    });

    Ok(result.to_string())
}
