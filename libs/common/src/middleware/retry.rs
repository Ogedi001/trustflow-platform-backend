//! Retry middleware
//!
//! Implements automatic retry logic with exponential backoff
//! for transient failures.

use std::time::Duration;

/// Retry configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retries
    pub max_retries: u32,
    /// Initial backoff duration
    pub initial_backoff: Duration,
    /// Maximum backoff duration
    pub max_backoff: Duration,
    /// Backoff multiplier (exponential backoff)
    pub backoff_multiplier: f64,
    /// Retry on specific status codes
    pub retryable_status_codes: Vec<u16>,
}

impl RetryConfig {
    /// Create new retry config
    pub fn new(max_retries: u32) -> Self {
        Self {
            max_retries,
            initial_backoff: Duration::from_millis(100),
            max_backoff: Duration::from_secs(30),
            backoff_multiplier: 2.0,
            retryable_status_codes: vec![
                408, // Request Timeout
                429, // Too Many Requests
                500, // Internal Server Error
                502, // Bad Gateway
                503, // Service Unavailable
                504, // Gateway Timeout
            ],
        }
    }

    /// Set initial backoff
    pub fn with_initial_backoff(mut self, backoff: Duration) -> Self {
        self.initial_backoff = backoff;
        self
    }

    /// Set maximum backoff
    pub fn with_max_backoff(mut self, backoff: Duration) -> Self {
        self.max_backoff = backoff;
        self
    }

    /// Set backoff multiplier
    pub fn with_backoff_multiplier(mut self, multiplier: f64) -> Self {
        self.backoff_multiplier = multiplier;
        self
    }

    /// Add retryable status code
    pub fn add_retryable_status(mut self, status: u16) -> Self {
        if !self.retryable_status_codes.contains(&status) {
            self.retryable_status_codes.push(status);
        }
        self
    }

    /// Check if status code is retryable
    pub fn is_retryable(&self, status: u16) -> bool {
        self.retryable_status_codes.contains(&status)
    }

    /// Calculate backoff for attempt
    pub fn calculate_backoff(&self, attempt: u32) -> Duration {
        let backoff_ms =
            self.initial_backoff.as_millis() as f64 * self.backoff_multiplier.powi(attempt as i32);
        let backoff_ms = backoff_ms.min(self.max_backoff.as_millis() as f64);
        Duration::from_millis(backoff_ms as u64)
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self::new(3)
    }
}

/// Retry state tracker
#[derive(Debug, Clone)]
pub struct RetryState {
    /// Current attempt
    pub attempt: u32,
    /// Total attempts made (including original)
    pub total_attempts: u32,
    /// Last error status if any
    pub last_error: Option<u16>,
}

impl RetryState {
    /// Create new retry state
    pub fn new() -> Self {
        Self {
            attempt: 0,
            total_attempts: 1,
            last_error: None,
        }
    }

    /// Increment attempt
    pub fn next_attempt(&mut self) {
        self.attempt += 1;
        self.total_attempts += 1;
    }

    /// Set last error
    pub fn set_error(&mut self, status: u16) {
        self.last_error = Some(status);
    }

    /// Check if should retry
    pub fn should_retry(&self, config: &RetryConfig) -> bool {
        self.attempt < config.max_retries
    }
}

impl Default for RetryState {
    fn default() -> Self {
        Self::new()
    }
}

/// Retry policy trait for custom retry logic
pub trait RetryPolicy: Send + Sync {
    /// Check if operation should be retried
    fn should_retry(&self, status: u16, attempt: u32) -> bool;

    /// Calculate backoff
    fn calculate_backoff(&self, attempt: u32) -> Duration;
}

/// Default retry policy
pub struct DefaultRetryPolicy {
    config: RetryConfig,
}

impl DefaultRetryPolicy {
    /// Create new default retry policy
    pub fn new(config: RetryConfig) -> Self {
        Self { config }
    }
}

impl RetryPolicy for DefaultRetryPolicy {
    fn should_retry(&self, status: u16, attempt: u32) -> bool {
        self.config.is_retryable(status) && attempt < self.config.max_retries
    }

    fn calculate_backoff(&self, attempt: u32) -> Duration {
        self.config.calculate_backoff(attempt)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_retry_config_creation() {
//         let config = RetryConfig::new(3);
//         assert_eq!(config.max_retries, 3);
//     }

//     #[test]
//     fn test_retry_config_is_retryable() {
//         let config = RetryConfig::new(3);
//         assert!(config.is_retryable(500));
//         assert!(config.is_retryable(503));
//         assert!(!config.is_retryable(400));
//     }

//     #[test]
//     fn test_retry_config_calculate_backoff() {
//         let config = RetryConfig::new(3);
//         let backoff_0 = config.calculate_backoff(0);
//         let backoff_1 = config.calculate_backoff(1);
//         assert!(backoff_1 > backoff_0);
//     }

//     #[test]
//     fn test_retry_state_creation() {
//         let state = RetryState::new();
//         assert_eq!(state.attempt, 0);
//         assert_eq!(state.total_attempts, 1);
//     }

//     #[test]
//     fn test_retry_state_next_attempt() {
//         let mut state = RetryState::new();
//         state.next_attempt();
//         assert_eq!(state.attempt, 1);
//         assert_eq!(state.total_attempts, 2);
//     }

//     #[test]
//     fn test_retry_state_should_retry() {
//         let config = RetryConfig::new(3);
//         let state = RetryState::new();
//         assert!(state.should_retry(&config));

//         let mut state = RetryState::new();
//         state.attempt = 3;
//         assert!(!state.should_retry(&config));
//     }
// }
