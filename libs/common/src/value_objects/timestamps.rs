//! Time-related value objects
//!
//! This module contains value objects for timestamps and durations.

use serde::{Deserialize, Serialize};
use time;

/// ISO 8601 timestamp wrapper using time crate
///
/// Represents a point in time with timezone awareness.
/// Always stored in UTC.
///
/// # Example
///
/// ```rust
/// use common::value_objects::Timestamp;
/// use std::time::Duration;
///
/// let now = Timestamp::now();
/// assert!(!now.is_future());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Timestamp(pub time::OffsetDateTime);

impl Timestamp {
    /// Get current UTC timestamp
    pub fn now() -> Self {
        Self(time::OffsetDateTime::now_utc())
    }

    /// Create from OffsetDateTime
    pub fn from_datetime(dt: time::OffsetDateTime) -> Self {
        Self(dt)
    }

    /// Get the inner OffsetDateTime value
    pub fn inner(&self) -> time::OffsetDateTime {
        self.0
    }

    /// Check if timestamp is in the past
    pub fn is_past(&self) -> bool {
        self.0 < time::OffsetDateTime::now_utc()
    }

    /// Check if timestamp is in the future
    pub fn is_future(&self) -> bool {
        self.0 > time::OffsetDateTime::now_utc()
    }

    /// Check if timestamp is approximately now (within a tolerance)
    pub fn is_now(&self, tolerance: time::Duration) -> bool {
        let now = time::OffsetDateTime::now_utc();
        (now - self.0).abs() <= tolerance
    }

    /// Get Unix timestamp (seconds since epoch)
    pub fn unix_timestamp(&self) -> i64 {
        self.0.unix_timestamp()
    }

    /// Get Unix timestamp in milliseconds
    pub fn unix_timestamp_millis(&self) -> i64 {
        self.0.unix_timestamp() * 1000 + self.0.millisecond() as i64
    }

    /// Format as RFC 3339 (ISO 8601)
    pub fn to_rfc3339(&self) -> String {
        self.0
            .format(time::macros::format_description!(
                "[year]-[month]-[day]T[hour]:[minute]:[second]Z"
            ))
            .unwrap_or_else(|_| "unknown".to_string())
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<time::OffsetDateTime> for Timestamp {
    fn from(dt: time::OffsetDateTime) -> Self {
        Self(dt)
    }
}

impl std::cmp::Ord for Timestamp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl std::cmp::PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Duration wrapper using time crate
///
/// Represents a span of time with convenient constructors.
///
/// # Example
///
/// ```rust
/// use common::value_objects::{Timestamp, Duration};
///
/// let expiry = Duration::hours(24).after(Timestamp::now());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Duration(pub time::Duration);

impl PartialOrd for Duration {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Duration {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl Duration {
    /// Create duration from seconds
    pub fn seconds(s: i64) -> Self {
        Self(time::Duration::seconds(s))
    }

    /// Create duration from milliseconds
    pub fn millis(ms: i64) -> Self {
        Self(time::Duration::milliseconds(ms))
    }

    /// Create duration from minutes
    pub fn minutes(m: i64) -> Self {
        Self(time::Duration::minutes(m))
    }

    /// Create duration from hours
    pub fn hours(h: i64) -> Self {
        Self(time::Duration::hours(h))
    }

    /// Create duration from days
    pub fn days(d: i64) -> Self {
        Self(time::Duration::days(d))
    }

    /// Get the inner Duration value
    pub fn inner(&self) -> time::Duration {
        self.0
    }

    /// Get total seconds
    pub fn total_seconds(&self) -> i64 {
        self.0.whole_seconds()
    }

    /// Get total milliseconds
    pub fn total_millis(&self) -> i128 {
        self.0.whole_milliseconds() as i128
    }

    /// Add this duration to a timestamp
    pub fn after(&self, base: Timestamp) -> Timestamp {
        Timestamp(base.0 + self.0)
    }

    /// Subtract this duration from a timestamp
    pub fn before(&self, base: Timestamp) -> Timestamp {
        Timestamp(base.0 - self.0)
    }

    /// Create a time range from now to this duration ahead
    pub fn from_now(&self) -> TimeRange {
        let now = Timestamp::now();
        TimeRange::new(now, self.after(now))
    }

    /// Create a time range from this duration ago to now
    pub fn ago(&self) -> TimeRange {
        let now = Timestamp::now();
        TimeRange::new(self.before(now), now)
    }
}

impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}s", self.0.whole_seconds())
    }
}

impl From<time::Duration> for Duration {
    fn from(d: time::Duration) -> Self {
        Self(d)
    }
}

impl std::ops::Add<Duration> for Timestamp {
    type Output = Timestamp;

    fn add(self, rhs: Duration) -> Timestamp {
        rhs.after(self)
    }
}

impl std::ops::Sub<Duration> for Timestamp {
    type Output = Timestamp;

    fn sub(self, rhs: Duration) -> Timestamp {
        rhs.before(self)
    }
}

/// Time range between two timestamps
///
/// Represents a span of time from `start` to `end`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeRange {
    /// Start of the time range (inclusive)
    pub start: Timestamp,
    /// End of the time range (exclusive)
    pub end: Timestamp,
}

impl TimeRange {
    /// Create a new time range
    ///
    /// Panics if end <= start
    pub fn new(start: Timestamp, end: Timestamp) -> Self {
        if end <= start {
            panic!("End time must be after start time");
        }
        Self { start, end }
    }

    /// Get the duration of this time range
    pub fn duration(&self) -> Duration {
        Duration(self.end.0 - self.start.0)
    }

    /// Check if a timestamp falls within this range
    pub fn contains(&self, ts: Timestamp) -> bool {
        ts >= self.start && ts < self.end
    }

    /// Check if this range overlaps with another
    pub fn overlaps(&self, other: TimeRange) -> bool {
        self.start < other.end && self.end > other.start
    }

    /// Get the intersection of two time ranges if they overlap
    pub fn intersection(&self, other: TimeRange) -> Option<TimeRange> {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);
        if start < end {
            Some(TimeRange { start, end })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_now() {
        let now = Timestamp::now();
        assert!(!now.is_future());
        assert!(!now.is_past() || now.is_now(time::Duration::seconds(1)));
    }

    #[test]
    fn test_duration_after() {
        let now = Timestamp::now();
        let future = Duration::hours(1).after(now);
        assert!(future.is_future());
    }

    #[test]
    fn test_duration_before() {
        let now = Timestamp::now();
        let past = Duration::hours(1).before(now);
        assert!(past.is_past());
    }

    #[test]
    fn test_timestamp_arithmetic() {
        let now = Timestamp::now();
        let later = now + Duration::seconds(100);
        assert!(later > now);
    }

    #[test]
    fn test_time_range() {
        let start = Timestamp::now();
        let end = Duration::hours(1).after(start);
        let range = TimeRange::new(start, end);
        assert_eq!(range.duration(), Duration::hours(1));
    }

    #[test]
    fn test_time_range_contains() {
        let start = Timestamp::now();
        let end = Duration::hours(1).after(start);
        let range = TimeRange::new(start, end);

        let middle = Duration::minutes(30).after(start);
        assert!(range.contains(middle));
    }

    #[test]
    fn test_duration_total_seconds() {
        let d = Duration::minutes(2);
        assert_eq!(d.total_seconds(), 120);
    }
}
