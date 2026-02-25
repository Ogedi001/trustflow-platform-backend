//! Core utility value objects
//!
//! This module contains utility types and markers for common patterns.

use serde::{Deserialize, Serialize};

/// Marker type for builder patterns and type-state pattern
///
/// Used to represent the initial state of a builder with no data.
///
/// # Example
///
/// ```rust
/// use common::value_objects::Empty;
///
/// struct UserBuilder<T = Empty> {
///     name: Option<String>,
///     _state: std::marker::PhantomData<T>,
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Empty;

impl Empty {
    /// Create a new Empty marker
    pub fn new() -> Self {
        Self
    }
}

/// Void/Unit type for functions that don't return meaningful data
///
/// Use this for endpoints or functions that succeed but don't have a body to return.
///
/// # Example
///
/// ```rust
/// use common::value_objects::Unit;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct DeleteResponse {
///     success: bool,
///     data: Unit,
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Unit(pub ());

impl Unit {
    /// Create a new Unit value
    pub fn new() -> Self {
        Self(())
    }
}

impl Default for Unit {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "()")
    }
}

/// Boolean flag wrapper for explicit intent
///
/// Use this when a boolean has semantic meaning in your domain.
/// E.g., `IsOptional`, `IsDeleted`, `IsVerified`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Flag(pub bool);

impl Flag {
    /// Create a true flag
    pub fn yes() -> Self {
        Self(true)
    }

    /// Create a false flag
    pub fn no() -> Self {
        Self(false)
    }

    /// Check if flag is set
    pub fn is_set(&self) -> bool {
        self.0
    }

    /// Check if flag is not set
    pub fn is_not_set(&self) -> bool {
        !self.0
    }
}

impl From<bool> for Flag {
    fn from(b: bool) -> Self {
        Self(b)
    }
}

impl std::fmt::Display for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Count wrapper for semantic type safety
///
/// Use when you want to distinguish between generic `u64` and actual count values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Count(pub u64);

impl Count {
    /// Create a count value
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    /// Get the count value
    pub fn value(&self) -> u64 {
        self.0
    }

    /// Check if count is zero
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    /// Check if count is non-zero
    pub fn is_nonzero(&self) -> bool {
        self.0 != 0
    }

    /// Increment the count
    pub fn increment(&mut self) {
        self.0 = self.0.saturating_add(1);
    }

    /// Decrement the count
    pub fn decrement(&mut self) {
        self.0 = self.0.saturating_sub(1);
    }
}

impl From<u64> for Count {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<usize> for Count {
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}

impl std::fmt::Display for Count {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit() {
        let unit = Unit::new();
        assert_eq!(unit, Unit::default());
        assert_eq!(unit.to_string(), "()");
    }

    #[test]
    fn test_flag() {
        let flag_yes = Flag::yes();
        let flag_no = Flag::no();

        assert!(flag_yes.is_set());
        assert!(flag_no.is_not_set());
    }

    #[test]
    fn test_count_operations() {
        let mut count = Count::new(5);
        count.increment();
        assert_eq!(count.value(), 6);

        count.decrement();
        assert_eq!(count.value(), 5);
    }

    #[test]
    fn test_count_saturation() {
        let mut count = Count::new(u64::MAX);
        count.increment(); // Should saturate, not overflow
        assert_eq!(count.value(), u64::MAX);

        let mut zero = Count::new(0);
        zero.decrement(); // Should saturate at 0
        assert_eq!(zero.value(), 0);
    }
}
