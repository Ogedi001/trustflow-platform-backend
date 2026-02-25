//! Observability utilities (tracing and metrics)

/// Structured logging and tracing utilities
pub struct Logging;

impl Logging {
    /// Emit a debug log message
    pub fn debug(msg: &str) {
        tracing::debug!("{}", msg);
    }

    /// Emit an info log message
    pub fn info(msg: &str) {
        tracing::info!("{}", msg);
    }

    /// Emit a warn log message
    pub fn warn(msg: &str) {
        tracing::warn!("{}", msg);
    }

    /// Emit an error log message
    pub fn error(msg: &str) {
        tracing::error!("{}", msg);
    }

    /// Log with fields
    pub fn with_context(level: LogLevel, msg: &str, fields: &[(&str, &str)]) {
        match level {
            LogLevel::Debug => {
                for (k, v) in fields {
                    tracing::debug!("{} {}={}", msg, k, v);
                }
            }
            LogLevel::Info => {
                for (k, v) in fields {
                    tracing::info!("{} {}={}", msg, k, v);
                }
            }
            LogLevel::Warn => {
                for (k, v) in fields {
                    tracing::warn!("{} {}={}", msg, k, v);
                }
            }
            LogLevel::Error => {
                for (k, v) in fields {
                    tracing::error!("{} {}={}", msg, k, v);
                }
            }
        }
    }
}

/// Log level enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    /// Debug level
    Debug,
    /// Info level
    Info,
    /// Warning level
    Warn,
    /// Error level
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level() {
        let levels = vec![LogLevel::Debug, LogLevel::Info, LogLevel::Warn, LogLevel::Error];
        assert_eq!(levels.len(), 4);
    }
}
