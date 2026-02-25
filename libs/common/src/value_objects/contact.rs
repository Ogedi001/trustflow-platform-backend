//! Contact information value objects
//!
//! This module contains value objects for contact details like email and phone numbers.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::Hash;

/// Email address value object with validation
///
/// Ensures email addresses conform to basic RFC 5322 standards.
/// Does not allow internationalized domain names by default.
///
/// # Example
///
/// ```rust
/// use common::value_objects::EmailAddress;
///
/// let email = EmailAddress::new("user@example.com").unwrap();
/// assert_eq!(email.domain(), Some("example.com"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EmailAddress(pub String);

impl EmailAddress {
    /// Create a new email address with validation
    ///
    /// Returns `Self` if valid, or an error string if invalid.
    pub fn new(email: impl Into<String>) -> Result<Self, String> {
        let email = email.into();
        if Self::is_valid_addr(&email) {
            Ok(Self(email))
        } else {
            Err(format!("Invalid email address: {}", email))
        }
    }

    /// Validate email format without creating an instance
    pub fn is_valid_addr(email: &str) -> bool {
        lazy_static::lazy_static! {
            static ref RE: regex::Regex = regex::Regex::new(
                r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
            ).unwrap();
        }
        RE.is_match(email)
    }

    /// Get domain part of email (e.g., "example.com" from "user@example.com")
    pub fn domain(&self) -> Option<&str> {
        self.0.split('@').nth(1)
    }

    /// Get local part of email (e.g., "user" from "user@example.com")
    pub fn local_part(&self) -> Option<&str> {
        self.0.split('@').next()
    }

    /// Get the email value as string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Phone number value object with international format support
///
/// Supports international phone numbers in E.164 format or with country-specific patterns.
/// Currently optimized for Nigerian numbers but handles general international formats.
///
/// # Example
///
/// ```rust
/// use common::value_objects::PhoneNumber;
///
/// let phone = PhoneNumber::new("+2348012345678").unwrap();
/// assert_eq!(phone.country_code(), Some("+234"));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PhoneNumber(pub String);

impl PhoneNumber {
    /// Create a new phone number with validation
    ///
    /// Validates that the number follows international format patterns.
    pub fn new(phone: impl Into<String>) -> Result<Self, String> {
        let phone = phone.into();
        if Self::is_valid_number(&phone) {
            Ok(Self(phone))
        } else {
            Err(format!("Invalid phone number: {}", phone))
        }
    }

    /// Validate phone number format without creating an instance
    pub fn is_valid_number(phone: &str) -> bool {
        lazy_static::lazy_static! {
            // Supports format: +[1-9]xxx...x (7-15 digits total)
            static ref RE: regex::Regex = regex::Regex::new(r"^\+?[1-9]\d{6,14}$").unwrap();
        }
        RE.is_match(phone)
    }

    /// Get country code if present (e.g., "+234" for Nigeria)
    pub fn country_code(&self) -> Option<&str> {
        if self.0.starts_with("+234") {
            Some("+234")
        } else if self.0.starts_with('+') {
            // For other country codes, extract up to the first digit
            self.0[1..].split(|c: char| c.is_ascii_digit()).next()
        } else {
            None
        }
    }

    /// Get national number (without country code)
    pub fn national_number(&self) -> Option<&str> {
        if self.0.starts_with("+234") {
            self.0.get(4..)
        } else if self.0.starts_with('+') {
            // Skip country code to get national number
            self.0.get(4..)
        } else {
            Some(&self.0)
        }
    }

    /// Get the phone value as string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Format to E.164 standard format
    pub fn to_e164(&self) -> String {
        self.0.clone() // Already in E.164 if valid
    }
}

impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        let email = EmailAddress::new("test@example.com").unwrap();
        assert_eq!(email.domain(), Some("example.com"));
        assert_eq!(email.local_part(), Some("test"));
    }

    #[test]
    fn test_invalid_email() {
        assert!(EmailAddress::new("invalid-email").is_err());
        assert!(EmailAddress::new("@example.com").is_err());
    }

    #[test]
    fn test_valid_nigerian_phone() {
        let phone = PhoneNumber::new("+2348012345678").unwrap();
        assert_eq!(phone.country_code(), Some("+234"));
    }

    #[test]
    fn test_invalid_phone() {
        assert!(PhoneNumber::new("123").is_err());
        assert!(PhoneNumber::new("abc").is_err());
    }
}
