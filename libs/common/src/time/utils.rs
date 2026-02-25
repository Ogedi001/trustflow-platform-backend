//! Time comparison and utility functions

use crate::value_objects::{Duration, Timestamp};

/// Time utility functions
pub struct TimeUtils;

impl TimeUtils {
    /// Get current timestamp
    pub fn now() -> Timestamp {
        Timestamp::now()
    }

    /// Check if timestamp is expired relative to now
    pub fn is_expired(ts: Timestamp) -> bool {
        ts.is_past()
    }

    /// Check if timestamp is valid (not too far in past or future)
    pub fn is_reasonable(ts: Timestamp, tolerance_hours: i64) -> bool {
        let tolerance = Duration::hours(tolerance_hours);
        let now = Timestamp::now();

        let max_past = tolerance.before(now);
        let max_future = tolerance.after(now);

        ts >= max_past && ts <= max_future
    }

    /// Format timestamp as RFC 3339
    pub fn format_rfc3339(ts: Timestamp) -> String {
        ts.to_rfc3339()
    }

    /// Get Unix timestamp
    pub fn to_unix(ts: Timestamp) -> i64 {
        ts.unix_timestamp()
    }

    /// Get time until timestamp expires
    pub fn time_until_expiry(expiry: Timestamp) -> Option<Duration> {
        let now = Timestamp::now();
        if expiry > now {
            Some(Duration(expiry.inner() - now.inner()))
        } else {
            None
        }
    }

    /// Check if two timestamps are within a tolerance duration
    pub fn within_tolerance(ts1: Timestamp, ts2: Timestamp, tolerance: Duration) -> bool {
        let diff = (ts1.inner() - ts2.inner()).abs();
        diff <= tolerance.inner()
    }
}

/// Timer for measuring elapsed time
pub struct Elapsed {
    start: Timestamp,
}

impl Elapsed {
    /// Create a new timer at current time
    pub fn now() -> Self {
        Self {
            start: Timestamp::now(),
        }
    }

    /// Create timer from specific timestamp
    pub fn from(ts: Timestamp) -> Self {
        Self { start: ts }
    }

    /// Get elapsed duration
    pub fn elapsed(&self) -> Duration {
        let now = Timestamp::now();
        Duration(now.inner() - self.start.inner())
    }

    /// Get elapsed seconds
    pub fn secs(&self) -> i64 {
        self.elapsed().total_seconds()
    }

    /// Get elapsed milliseconds
    pub fn millis(&self) -> i128 {
        self.elapsed().total_millis()
    }

    /// Check if duration has elapsed
    pub fn has_elapsed(&self, duration: Duration) -> bool {
        self.elapsed() >= duration
    }

    /// Reset timer to current time
    pub fn reset(&mut self) {
        self.start = Timestamp::now();
    }

    /// Get start timestamp
    pub fn start(&self) -> Timestamp {
        self.start
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration as StdDuration;

    #[test]
    fn test_is_expired() {
        let past = Duration::hours(1).before(Timestamp::now());
        assert!(TimeUtils::is_expired(past));
    }

    #[test]
    fn test_elapsed_timer() {
        let timer = Elapsed::now();
        thread::sleep(StdDuration::from_millis(100));
        assert!(timer.secs() >= 0);
        assert!(timer.millis() > 50);
    }

    #[test]
    fn test_has_elapsed() {
        let timer = Elapsed::now();
        assert!(!timer.has_elapsed(Duration::hours(1)));
    }

    #[test]
    fn test_within_tolerance() {
        let ts1 = Timestamp::now();
        let ts2 = Duration::seconds(1).after(ts1);
        assert!(TimeUtils::within_tolerance(ts1, ts2, Duration::seconds(2)));
    }
}
