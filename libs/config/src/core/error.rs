//! Configuration error types
//!
//! Provides error types for configuration loading, validation, and parsing.
use thiserror::Error;

pub type ConfigResult<T> = Result<T, ConfigError>;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to load configuration: {0}")]
    Load(String),

    #[error("Failed to parse configuration key '{key}': {reason}")]
    Parse { key: String, reason: String },

    #[error("Required configuration key '{key}' is missing")]
    Missing { key: String },

    #[error("Configuration validation failed: {0}")]
    Validation(String),

    #[error("Invalid configuration value for '{key}': {reason}")]
    InvalidValue { key: String, reason: String },

    #[error("Environment variable '{name}' is not set")]
    EnvVarNotSet { name: String },

    #[error("Failed to read file '{path}': {source}")]
    FileRead {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to parse YAML from '{path}': {source}")]
    YamlParse {
        path: String,
        #[source]
        source: serde_yaml::Error,
    },

    #[error("Failed to parse JSON from '{path}': {source}")]
    JsonParse {
        path: String,
        #[source]
        source: serde_json::Error,
    },

    #[error("Configuration source error: {0}")]
    Source(String),

    #[error("Feature '{feature}' is not enabled")]
    FeatureNotEnabled { feature: String },
}

impl ConfigError {
    pub fn parse(key: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::Parse {
            key: key.into(),
            reason: reason.into(),
        }
    }

    /// Create a new missing key error
    pub fn missing(key: impl Into<String>) -> Self {
        Self::Missing { key: key.into() }
    }

    /// Create a new invalid value error
    pub fn invalid_value(key: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidValue {
            key: key.into(),
            reason: reason.into(),
        }
    }

    /// Create a new validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation(message.into())
    }

    /// Create a new environment variable not set error
    pub fn env_var_not_set(name: impl Into<String>) -> Self {
        Self::EnvVarNotSet { name: name.into() }
    }

    /// Create a new file read error
    pub fn file_read(path: impl Into<String>, source: std::io::Error) -> Self {
        Self::FileRead {
            path: path.into(),
            source,
        }
    }

    /// Create a new YAML parse error
    pub fn yaml_parse(path: impl Into<String>, source: serde_yaml::Error) -> Self {
        Self::YamlParse {
            path: path.into(),
            source,
        }
    }

    /// Create a new JSON parse error
    pub fn json_parse(path: impl Into<String>, source: serde_json::Error) -> Self {
        Self::JsonParse {
            path: path.into(),
            source,
        }
    }

    /// Create a new source error
    pub fn source(message: impl Into<String>) -> Self {
        Self::Source(message.into())
    }

    /// Check if this is a missing key error
    pub fn is_missing(&self) -> bool {
        matches!(self, Self::Missing { .. })
    }

    /// Check if this is a validation error
    pub fn is_validation(&self) -> bool {
        matches!(self, Self::Validation(..))
    }
}

/// Extension trait for adding context to config errors
pub trait ConfigErrorExt {
    /// Add context to a config error
    fn context(self, key: impl Into<String>) -> ConfigError;
}

impl ConfigErrorExt for ConfigError {
    fn context(self, key: impl Into<String>) -> ConfigError {
        let key = key.into();
        match self {
            ConfigError::Parse { reason, .. } => ConfigError::Parse { key, reason },
            ConfigError::InvalidValue { reason, .. } => ConfigError::InvalidValue { key, reason },
            other => other,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_config_error_creation() {
//         let err = ConfigError::missing("DATABASE_URL");
//         assert!(err.is_missing());

//         let err = ConfigError::validation("Invalid configuration");
//         assert!(err.is_validation());

//         let err = ConfigError::parse("port", "not a valid port number");
//         assert!(matches!(err, ConfigError::Parse { .. }));
//     }

// #[test]
// fn test_config_error_context() {
//     let err = ConfigError::parse("value", "not a number").context("server.port");
//     assert!(matches!(
//         err,
//         ConfigError::Parse {
//             key,
//             reason: _
//         } if key == "server.port"
//     ));
// }
//}
