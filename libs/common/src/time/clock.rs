//! Clock and current time utilities

use crate::value_objects::timestamps::Timestamp;

/// System clock for getting current time
pub struct Clock;

impl Clock {
    /// Get current timestamp in UTC
    pub fn now() -> Timestamp {
        Timestamp::now()
    }

    /// Get current Unix timestamp (seconds since epoch)
    pub fn unix_now() -> i64 {
        Timestamp::now().unix_timestamp()
    }

    /// Get current Unix timestamp in milliseconds
    pub fn unix_now_millis() -> i64 {
        Timestamp::now().unix_timestamp_millis()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clock_now() {
        let now = Clock::now();
        assert!(!now.is_future());
    }

    #[test]
    fn test_unix_now() {
        let unix = Clock::unix_now();
        assert!(unix > 0);
    }
}
