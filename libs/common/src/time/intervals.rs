//! Interval and periodic execution utilities

use crate::value_objects::timestamps::{Duration, Timestamp};

/// Interval for periodic tasks
#[derive(Debug, Clone)]
pub struct Interval {
    period: Duration,
    next_trigger: Timestamp,
}

impl Interval {
    /// Create a new interval with given period
    pub fn new(period: Duration) -> Self {
        Self {
            period,
            next_trigger: Timestamp::now(),
        }
    }

    /// Check if the interval has elapsed
    pub fn is_ready(&self) -> bool {
        Timestamp::now() >= self.next_trigger
    }

    /// Check if ready and advance to next trigger
    pub fn tick(&mut self) -> bool {
        if self.is_ready() {
            self.next_trigger = self.period.after(self.next_trigger);
            true
        } else {
            false
        }
    }

    /// Reset the interval
    pub fn reset(&mut self) {
        self.next_trigger = self.period.after(Timestamp::now());
    }

    /// Get time until next trigger
    pub fn time_until_ready(&self) -> Option<Duration> {
        let now = Timestamp::now();
        if self.next_trigger > now {
            Some(Duration(self.next_trigger.inner() - now.inner()))
        } else {
            None
        }
    }

    /// Get the interval period
    pub fn period(&self) -> Duration {
        self.period
    }
}

/// Rate limiter based on time windows
#[derive(Debug, Clone)]
pub struct RateWindow {
    window_size: Duration,
    max_events: u32,
    last_reset: Timestamp,
    event_count: u32,
}

impl RateWindow {
    /// Create a new rate window
    pub fn new(window_size: Duration, max_events: u32) -> Self {
        Self {
            window_size,
            max_events,
            last_reset: Timestamp::now(),
            event_count: 0,
        }
    }

    /// Check and record an event
    pub fn allow_event(&mut self) -> bool {
        let now = Timestamp::now();

        // Reset window if expired
        if now > self.window_size.after(self.last_reset) {
            self.last_reset = now;
            self.event_count = 0;
        }

        // Check if under limit
        if self.event_count < self.max_events {
            self.event_count += 1;
            true
        } else {
            false
        }
    }

    /// Get current event count
    pub fn current_count(&self) -> u32 {
        self.event_count
    }

    /// Get events remaining
    pub fn remaining(&self) -> u32 {
        self.max_events.saturating_sub(self.event_count)
    }

    /// Check if at limit
    pub fn is_exhausted(&self) -> bool {
        self.event_count >= self.max_events
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_ready() {
        let mut interval = Interval::new(Duration::millis(1));
        assert!(interval.tick());
    }

    #[test]
    fn test_rate_window() {
        let mut window = RateWindow::new(Duration::seconds(1), 3);

        assert!(window.allow_event());
        assert!(window.allow_event());
        assert!(window.allow_event());
        assert!(!window.allow_event()); // Exceeded limit

        assert_eq!(window.current_count(), 3);
        assert_eq!(window.remaining(), 0);
        assert!(window.is_exhausted());
    }
}
