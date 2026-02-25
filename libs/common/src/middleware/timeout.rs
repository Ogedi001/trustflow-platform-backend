//! Request timeout middleware
//!
//! Enforces maximum request/operation durations to prevent
//! resources from being held indefinitely.

use std::time::Duration;

/// Timeout configuration
#[derive(Debug, Clone)]
pub struct TimeoutConfig {
    /// Default timeout duration
    pub default_timeout: Duration,
    /// Maximum timeout (cannot be exceeded)
    pub max_timeout: Duration,
    /// Path-specific timeouts
    pub path_timeouts: Vec<(String, Duration)>,
}

impl TimeoutConfig {
    /// Create new timeout config
    pub fn new(default_timeout: Duration) -> Self {
        Self {
            default_timeout,
            max_timeout: Duration::from_secs(300), // 5 minutes
            path_timeouts: Vec::new(),
        }
    }

    /// Set maximum timeout
    pub fn with_max_timeout(mut self, timeout: Duration) -> Self {
        self.max_timeout = timeout;
        self
    }

    /// Add path-specific timeout
    pub fn add_path_timeout(mut self, path: impl Into<String>, timeout: Duration) -> Self {
        let timeout = timeout.min(self.max_timeout);
        self.path_timeouts.push((path.into(), timeout));
        self
    }

    /// Get timeout for path
    pub fn get_timeout(&self, path: &str) -> Duration {
        for (pattern, timeout) in &self.path_timeouts {
            if path.starts_with(pattern) {
                return *timeout;
            }
        }
        self.default_timeout
    }
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self::new(Duration::from_secs(30))
    }
}

/// Standard timeout presets
pub mod presets {
    use super::*;

    /// Aggressive: 5 second timeout
    pub fn aggressive() -> TimeoutConfig {
        TimeoutConfig::new(Duration::from_secs(5))
    }

    /// Standard: 30 second timeout
    pub fn standard() -> TimeoutConfig {
        TimeoutConfig::new(Duration::from_secs(30))
    }

    /// Lenient: 120 second timeout
    pub fn lenient() -> TimeoutConfig {
        TimeoutConfig::new(Duration::from_secs(120))
    }

    /// Very lenient: 600 second (10 minute) timeout
    pub fn very_lenient() -> TimeoutConfig {
        TimeoutConfig::new(Duration::from_secs(600))
    }

    /// Custom by duration
    pub fn custom(duration: Duration) -> TimeoutConfig {
        TimeoutConfig::new(duration)
    }
}

/// Timeout violation action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeoutAction {
    /// Return 408 Request Timeout
    RequestTimeout,
    /// Return 504 Gateway Timeout
    GatewayTimeout,
    /// Forcefully abort the request
    Abort,
}

impl Default for TimeoutAction {
    fn default() -> Self {
        Self::RequestTimeout
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_timeout_config_creation() {
//         let config = TimeoutConfig::new(Duration::from_secs(30));
//         assert_eq!(config.default_timeout, Duration::from_secs(30));
//     }

//     #[test]
//     fn test_timeout_config_get_timeout() {
//         let config = TimeoutConfig::new(Duration::from_secs(30))
//             .add_path_timeout("/api/upload", Duration::from_secs(120));

//         assert_eq!(
//             config.get_timeout("/api/upload/file"),
//             Duration::from_secs(120)
//         );
//         assert_eq!(
//             config.get_timeout("/api/users"),
//             Duration::from_secs(30)
//         );
//     }

//     #[test]
//     fn test_timeout_config_max_timeout_respected() {
//         let config = TimeoutConfig::new(Duration::from_secs(30))
//             .with_max_timeout(Duration::from_secs(60));

//         // Trying to add timeout longer than max should be capped
//         let config = config.add_path_timeout("/long", Duration::from_secs(300));
//         assert_eq!(config.get_timeout("/long"), Duration::from_secs(60));
//     }

//     #[test]
//     fn test_timeout_presets() {
//         assert_eq!(presets::aggressive().default_timeout, Duration::from_secs(5));
//         assert_eq!(presets::standard().default_timeout, Duration::from_secs(30));
//         assert_eq!(
//             presets::lenient().default_timeout,
//             Duration::from_secs(120)
//         );
//     }

//     #[test]
//     fn test_timeout_action_default() {
//         assert_eq!(TimeoutAction::default(), TimeoutAction::RequestTimeout);
//     }
// }
