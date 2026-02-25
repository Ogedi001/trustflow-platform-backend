//! Validation rules and constraints
//!
//! This module provides reusable validation rules for common domain constraints.

use crate::value_objects::{EmailAddress, PhoneNumber};

/// Result type for validation operations
pub type ValidationResult<T> = Result<T, ValidationError>;

/// Validation error wrapper
///
/// Accumulates multiple validation errors that can be returned together.
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// Field name that failed validation
    pub field: String,
    /// Error message describing the validation failure
    pub message: String,
}

impl ValidationError {
    /// Create a new validation error
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

impl std::error::Error for ValidationError {}

/// Collection of validation errors
#[derive(Debug, Clone, Default)]
pub struct ValidationErrors {
    errors: Vec<ValidationError>,
}

impl ValidationErrors {
    /// Create empty validation errors collection
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an error
    pub fn add(&mut self, field: impl Into<String>, message: impl Into<String>) {
        self.errors.push(ValidationError::new(field, message));
    }

    /// Add an error and return self for chaining
    pub fn with_error(mut self, field: impl Into<String>, message: impl Into<String>) -> Self {
        self.add(field, message);
        self
    }

    /// Check if there are any errors
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Get all errors
    pub fn as_slice(&self) -> &[ValidationError] {
        &self.errors
    }

    /// Convert to a result
    pub fn into_result(self) -> Result<(), ValidationErrors> {
        if self.is_empty() {
            Ok(())
        } else {
            Err(self)
        }
    }

    /// Get first error if any
    pub fn first(&self) -> Option<&ValidationError> {
        self.errors.first()
    }

    /// Get count of errors
    pub fn len(&self) -> usize {
        self.errors.len()
    }
}

impl std::fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let messages = self
            .errors
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{}", messages)
    }
}

impl std::error::Error for ValidationErrors {}

impl From<ValidationError> for ValidationErrors {
    fn from(error: ValidationError) -> Self {
        let mut errors = ValidationErrors::new();
        errors.errors.push(error);
        errors
    }
}

impl<I> FromIterator<I> for ValidationErrors
where
    I: Into<ValidationError>,
{
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        let mut errors = ValidationErrors::new();
        for item in iter {
            errors.errors.push(item.into());
        }
        errors
    }
}

/// String validation rules
pub struct StringRules;

impl StringRules {
    /// Validate string is not empty or whitespace
    pub fn not_empty(value: &str, field: &str) -> ValidationResult<()> {
        if value.trim().is_empty() {
            Err(ValidationError::new(field, "cannot be empty"))
        } else {
            Ok(())
        }
    }

    /// Validate string length is within range
    pub fn length_range(value: &str, min: usize, max: usize, field: &str) -> ValidationResult<()> {
        let len = value.len();
        if len < min || len > max {
            Err(ValidationError::new(
                field,
                format!("length must be between {} and {}", min, max),
            ))
        } else {
            Ok(())
        }
    }

    /// Validate string length is at least min
    pub fn min_length(value: &str, min: usize, field: &str) -> ValidationResult<()> {
        if value.len() < min {
            Err(ValidationError::new(
                field,
                format!("must be at least {} characters", min),
            ))
        } else {
            Ok(())
        }
    }

    /// Validate string length is at most max
    pub fn max_length(value: &str, max: usize, field: &str) -> ValidationResult<()> {
        if value.len() > max {
            Err(ValidationError::new(
                field,
                format!("must be at most {} characters", max),
            ))
        } else {
            Ok(())
        }
    }

    /// Validate string matches regex pattern
    pub fn matches_pattern(
        value: &str,
        pattern: &regex::Regex,
        field: &str,
    ) -> ValidationResult<()> {
        if pattern.is_match(value) {
            Ok(())
        } else {
            Err(ValidationError::new(
                field,
                "does not match required pattern",
            ))
        }
    }
}

/// Email validation rules
pub struct EmailRules;

impl EmailRules {
    /// Validate email address format
    pub fn valid_email(email: &str, field: &str) -> ValidationResult<()> {
        if EmailAddress::is_valid_addr(email) {
            Ok(())
        } else {
            Err(ValidationError::new(field, "invalid email address"))
        }
    }

    /// Create validated email
    pub fn create_email(email: &str, field: &str) -> ValidationResult<EmailAddress> {
        EmailAddress::new(email)
            .map_err(|_| ValidationError::new(field, "invalid email address"))
    }
}

/// Phone validation rules
pub struct PhoneRules;

impl PhoneRules {
    /// Validate phone number format
    pub fn valid_phone(phone: &str, field: &str) -> ValidationResult<()> {
        if PhoneNumber::is_valid_number(phone) {
            Ok(())
        } else {
            Err(ValidationError::new(field, "invalid phone number"))
        }
    }

    /// Create validated phone
    pub fn create_phone(phone: &str, field: &str) -> ValidationResult<PhoneNumber> {
        PhoneNumber::new(phone)
            .map_err(|_| ValidationError::new(field, "invalid phone number"))
    }
}

/// Number validation rules
pub struct NumberRules;

impl NumberRules {
    /// Validate number is positive
    pub fn is_positive(value: i64, field: &str) -> ValidationResult<()> {
        if value > 0 {
            Ok(())
        } else {
            Err(ValidationError::new(field, "must be positive"))
        }
    }

    /// Validate number is non-negative
    pub fn is_non_negative(value: i64, field: &str) -> ValidationResult<()> {
        if value >= 0 {
            Ok(())
        } else {
            Err(ValidationError::new(field, "must be non-negative"))
        }
    }

    /// Validate number is within range
    pub fn in_range(value: i64, min: i64, max: i64, field: &str) -> ValidationResult<()> {
        if value < min || value > max {
            Err(ValidationError::new(
                field,
                format!("must be between {} and {}", min, max),
            ))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_not_empty() {
        assert!(StringRules::not_empty("test", "field").is_ok());
        assert!(StringRules::not_empty("", "field").is_err());
        assert!(StringRules::not_empty("   ", "field").is_err());
    }

    #[test]
    fn test_string_length() {
        assert!(StringRules::length_range("test", 2, 10, "field").is_ok());
        assert!(StringRules::length_range("t", 2, 10, "field").is_err());
        assert!(StringRules::length_range("toolong", 2, 5, "field").is_err());
    }

    #[test]
    fn test_email_validation() {
        assert!(EmailRules::valid_email("test@example.com", "email").is_ok());
        assert!(EmailRules::valid_email("invalid-email", "email").is_err());
    }

    #[test]
    fn test_phone_validation() {
        assert!(PhoneRules::valid_phone("+2348012345678", "phone").is_ok());
        assert!(PhoneRules::valid_phone("123", "phone").is_err());
    }

    #[test]
    fn test_validation_errors() {
        let mut errors = ValidationErrors::new();
        errors.add("email", "invalid email");
        errors.add("phone", "invalid phone");

        assert_eq!(errors.len(), 2);
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_number_validation() {
        assert!(NumberRules::is_positive(5, "field").is_ok());
        assert!(NumberRules::is_positive(-1, "field").is_err());
        assert!(NumberRules::is_non_negative(0, "field").is_ok());
    }
}
