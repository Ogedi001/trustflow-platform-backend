//! Network-related value objects
//!
//! This module contains value objects for network identifiers like URLs and IP addresses.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::Hash;
use std::str::FromStr;

/// URL wrapper value object for type safety
///
/// Ensures URLs are valid according to basic standards.
///
/// # Example
///
/// ```rust
/// use common::value_objects::Url;
///
/// let url = Url::new("https://example.com/path");
/// assert_eq!(url.as_str(), "https://example.com/path");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Url(pub String);

impl Url {
    /// Create a new URL with basic validation
    pub fn new(url: impl Into<String>) -> Result<Self, String> {
        let url = url.into();
        if Self::is_valid_url(&url) {
            Ok(Self(url))
        } else {
            Err(format!("Invalid URL: {}", url))
        }
    }

    /// Validate URL format (basic check)
    pub fn is_valid_url(url: &str) -> bool {
        url.starts_with("http://") || url.starts_with("https://") || url.starts_with("ftp://")
    }

    /// Get the URL value as string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Get protocol (http, https, ftp)
    pub fn protocol(&self) -> Option<&str> {
        if let Some(pos) = self.0.find("://") {
            Some(&self.0[..pos])
        } else {
            None
        }
    }

    /// Get host from URL
    pub fn host(&self) -> Option<&str> {
        if let Some(start) = self.0.find("://") {
            let rest = &self.0[start + 3..];
            rest.split('/')
                .next()
                .and_then(|h| h.split(':').next())
                .or(Some(rest.split('/').next()?))
        } else {
            None
        }
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for Url {
    fn from(url: String) -> Self {
        Self(url)
    }
}

/// IP Address wrapper for type safety
///
/// Supports both IPv4 and IPv6 addresses.
///
/// # Example
///
/// ```rust
/// use common::value_objects::IpAddress;
///
/// let ip = IpAddress::new("192.168.1.1").unwrap();
/// assert_eq!(ip.as_str(), "192.168.1.1");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IpAddress(pub String);

impl IpAddress {
    /// Create a new IP address with validation
    pub fn new(ip: impl Into<String>) -> Result<Self, String> {
        let ip = ip.into();
        if Self::is_valid_ip(&ip) {
            Ok(Self(ip))
        } else {
            Err(format!("Invalid IP address: {}", ip))
        }
    }

    /// Validate IP address format
    pub fn is_valid_ip(ip: &str) -> bool {
        ip.parse::<std::net::IpAddr>().is_ok()
    }

    /// Get the IP address value
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Check if this is an IPv4 address
    pub fn is_ipv4(&self) -> bool {
        self.0.parse::<std::net::Ipv4Addr>().is_ok()
    }

    /// Check if this is an IPv6 address
    pub fn is_ipv6(&self) -> bool {
        self.0.parse::<std::net::Ipv6Addr>().is_ok()
    }

    /// Check if this is a private IP address
    pub fn is_private(&self) -> bool {
        if let Ok(ip) = self.0.parse::<std::net::IpAddr>() {
            match ip {
                std::net::IpAddr::V4(ipv4) => ipv4.is_private(),
                std::net::IpAddr::V6(ipv6) => ipv6.is_unique_local(),
            }
        } else {
            false
        }
    }

    /// Check if this is localhost
    pub fn is_localhost(&self) -> bool {
        if let Ok(ip) = self.0.parse::<std::net::IpAddr>() {
            ip.is_loopback()
        } else {
            false
        }
    }
}

impl fmt::Display for IpAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for IpAddress {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

/// User Agent wrapper for HTTP client identification
///
/// Stores the User-Agent header value from HTTP requests.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserAgent(pub String);

impl UserAgent {
    /// Create from a user agent string
    pub fn new(ua: impl Into<String>) -> Self {
        Self(ua.into())
    }

    /// Get the user agent string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for UserAgent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_url() {
        let url = Url::new("https://example.com").unwrap();
        assert_eq!(url.protocol(), Some("https"));
        assert_eq!(url.host(), Some("example.com"));
    }

    #[test]
    fn test_invalid_url() {
        assert!(Url::new("not-a-url").is_err());
    }

    #[test]
    fn test_valid_ipv4() {
        let ip = IpAddress::new("192.168.1.1").unwrap();
        assert!(ip.is_ipv4());
        assert!(!ip.is_ipv6());
        assert!(ip.is_private());
    }

    #[test]
    fn test_localhost() {
        let ip = IpAddress::new("127.0.0.1").unwrap();
        assert!(ip.is_localhost());
    }

    #[test]
    fn test_invalid_ip() {
        assert!(IpAddress::new("256.256.256.256").is_err());
    }
}
