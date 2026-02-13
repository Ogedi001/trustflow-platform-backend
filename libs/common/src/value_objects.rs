//! Shared value objects for Identity Service that can be used across all services
//!
//! These are generic value objects that don't contain service-specific logic.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::Hash;
use std::str::FromStr;

/// User ID newtype wrapper for type safety across services
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub uuid::Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub fn as_uuid(&self) -> uuid::Uuid {
        self.0
    }

    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<uuid::Uuid> for UserId {
    fn from(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }
}

impl FromStr for UserId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        uuid::Uuid::parse_str(s).map(Self)
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Email address value object with validation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EmailAddress(pub String);

impl EmailAddress {
    /// Validate email format
    pub fn is_valid(&self) -> bool {
        lazy_static::lazy_static! {
            static ref RE: regex::Regex = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        }
        RE.is_match(&self.0)
    }

    /// Get domain part of email
    pub fn domain(&self) -> Option<&str> {
        self.0.split('@').nth(1)
    }

    /// Get the email value
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Phone number value object with Nigerian format support
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PhoneNumber(pub String);

impl PhoneNumber {
    /// Validate Nigerian phone number format
    pub fn is_valid(&self) -> bool {
        lazy_static::lazy_static! {
            static ref RE: regex::Regex = regex::Regex::new(r"^\+?[1-9]\d{6,14}$").unwrap();
        }
        RE.is_match(&self.0)
    }

    /// Get country code
    pub fn country_code(&self) -> Option<&str> {
        if self.0.starts_with("+234") {
            Some("+234")
        } else if self.0.starts_with('+') {
            self.0.split_at(1).0
        } else {
            None
        }
    }

    /// Get national number (without country code)
    pub fn national_number(&self) -> Option<&str> {
        if self.0.starts_with('+') {
            self.0.get(4..)
        } else {
            Some(&self.0)
        }
    }

    /// Get the phone value
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Password hash wrapper
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasswordHash(pub String);

impl PasswordHash {
    /// Create from hash string
    pub fn from_hash(hash: &str) -> Self {
        Self(hash.to_string())
    }

    /// Get the hash value
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// URL wrapper for type safety
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Url(pub String);

impl Url {
    /// Get the URL value
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Secret wrapper for sensitive data
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Secret(pub String);

impl Secret {
    /// Get the secret value (use carefully)
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// IP Address wrapper
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IpAddress(pub String);

impl IpAddress {
    /// Get the IP address value
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Device ID wrapper
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeviceId(pub String);

impl DeviceId {
    /// Get the device ID value
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Pagination parameters - shared across all services
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pagination {
    pub page: u32,
    pub limit: u32,
}

impl Pagination {
    /// Create pagination with defaults
    pub fn new(page: u32, limit: u32) -> Self {
        Self {
            page: page.max(1),
            limit: limit.min(100).max(1),
        }
    }

    /// Default pagination (page 1, limit 20)
    pub fn default() -> Self {
        Self::new(1, 20)
    }

    /// Get offset for database queries
    pub fn offset(&self) -> u64 {
        ((self.page - 1) * self.limit) as u64
    }

    /// Get limit
    pub fn limit(&self) -> u32 {
        self.limit
    }

    /// Get page
    pub fn page(&self) -> u32 {
        self.page
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self::default()
    }
}

/// Sort direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SortDirection {
    Asc,
    Desc,
}

/// Sorting parameters
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sort<T: ToString + Clone> {
    pub field: T,
    pub direction: SortDirection,
}

impl<T: ToString + Clone> Sort<T> {
    /// Create ascending sort
    pub fn asc(field: T) -> Self {
        Self {
            field,
            direction: SortDirection::Asc,
        }
    }

    /// Create descending sort
    pub fn desc(field: T) -> Self {
        Self {
            field,
            direction: SortDirection::Desc,
        }
    }
}

/// Search parameters combining pagination, sort, and filters
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SearchParams<T: ToString + Clone> {
    pub pagination: Pagination,
    pub sorts: Vec<Sort<T>>,
    pub query: Option<String>,
}

impl<T: ToString + Clone> SearchParams<T> {
    /// Create search params with defaults
    pub fn new() -> Self {
        Self {
            pagination: Pagination::default(),
            sorts: Vec::new(),
            query: None,
        }
    }

    /// Add sort
    pub fn sort(mut self, sort: Sort<T>) -> Self {
        self.sorts.push(sort);
        self
    }

    /// Set query
    pub fn query(mut self, query: String) -> Self {
        self.query = Some(query);
        self
    }
}

/// ISO 8601 timestamp wrapper using time crate
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Timestamp(pub time::OffsetDateTime);

impl Timestamp {
    /// Get current UTC timestamp
    pub fn now() -> Self {
        Self(time::OffsetDateTime::now_utc())
    }

    /// Get the inner value
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
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

/// Duration wrapper using time crate
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Duration(pub time::Duration);

impl Duration {
    /// Create duration from seconds
    pub fn seconds(s: i64) -> Self {
        Self(time::Duration::seconds(s))
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

    /// Get the inner value
    pub fn inner(&self) -> time::Duration {
        self.0
    }

    /// Add to a timestamp
    pub fn after(&self, base: Timestamp) -> Timestamp {
        Timestamp(base.0 + self.0)
    }

    /// Subtract from a timestamp
    pub fn before(&self, base: Timestamp) -> Timestamp {
        Timestamp(base.0 - self.0)
    }
}

/// Empty struct for builder patterns
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Empty;

/// Unit type for void returns
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Unit(pub ());

impl Unit {
    pub fn new() -> Self {
        Self(())
    }
}

impl Default for Unit {
    fn default() -> Self {
        Self::new()
    }
}
