//! Time utilities module
//!
//! Provides time-related utilities, timers, intervals, and scheduling helpers.
//!
//! ## Organization
//!
//! - `clock` - System clock and current time
//! - `intervals` - Periodic intervals and rate limiting
//! - `utils` - Time comparison and duration utilities

mod clock;
mod intervals;
mod utils;

pub use clock::Clock;
pub use intervals::{Interval, RateWindow};
pub use utils::{Elapsed, TimeUtils};

/// Prelude for time module
pub mod prelude {
    //! Import common time items with `use common::time::prelude::*;`
    pub use super::{Clock, Elapsed, Interval, RateWindow, TimeUtils};
}
