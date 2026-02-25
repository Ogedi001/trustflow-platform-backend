//! Request-level validation utilities
//!
//! This module provides traits and utilities for validating entire requests.

use super::rules::{ValidationErrors, ValidationResult};
#[cfg(test)]
use super::rules::ValidationError;

/// Trait for types that can be validated
///
/// Implementing this trait allows for comprehensive validation of aggregate objects.
/// # Example
///
/// ```rust
/// use common::validation::{Validate, ValidationErrors};
///
/// #[derive(Clone)]
/// struct CreateUserRequest {
///     email: String,
///     name: String,
/// }
///
/// impl Validate for CreateUserRequest {
///     fn validate(&self) -> Result<(), ValidationErrors> {
///         let mut errors = ValidationErrors::new();
///         
///         if self.email.is_empty() {
///             errors.add("email", "required");
///         }
///         
///         if self.name.is_empty() {
///             errors.add("name", "required");
///         }
///
///         errors.into_result()
///     }
/// }
/// ```
pub trait Validate {
    /// Validate this object
    ///
    /// Should return `Ok(())` if valid, or `Err(ValidationErrors)` with all errors found.
    fn validate(&self) -> Result<(), ValidationErrors>;
}

/// Trait for types with conditional validation
///
/// Use this when validation rules depend on context or state.
pub trait ValidateWith {
    /// The context type for validation
    type Context;

    /// Validate with context
    fn validate_with(&self, context: &Self::Context) -> Result<(), ValidationErrors>;
}

/// Validator helper for composing validation operations
pub struct RequestValidator {
    errors: ValidationErrors,
}

impl RequestValidator {
    /// Create a new validator
    pub fn new() -> Self {
        Self {
            errors: ValidationErrors::new(),
        }
    }

    /// Add an error to the validator
    pub fn add_error(&mut self, field: impl Into<String>, message: impl Into<String>) {
        self.errors.add(field, message);
    }

    /// Add error if condition is true
    pub fn add_error_if(
        &mut self,
        condition: bool,
        field: impl Into<String>,
        message: impl Into<String>,
    ) {
        if condition {
            self.add_error(field, message);
        }
    }

    /// Validate a field using a closure
    pub fn validate_field<F>(
        &mut self,
        field: impl Into<String>,
        validator: F,
    ) -> &mut Self
    where
        F: FnOnce() -> Result<(), String>,
    {
        let field = field.into();
        if let Err(msg) = validator() {
            self.add_error(field, msg);
        }
        self
    }

    /// Merge errors from another validation result
    pub fn merge(&mut self, errors: ValidationErrors) {
        for error in errors.as_slice() {
            self.add_error(error.field.clone(), error.message.clone());
        }
    }

    /// Check if there are any validation errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Convert to result
    pub fn into_result(self) -> Result<(), ValidationErrors> {
        self.errors.into_result()
    }

    /// Get the collected errors
    pub fn errors(&self) -> &ValidationErrors {
        &self.errors
    }
}

impl Default for RequestValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for chained validations
pub struct ValidationBuilder<T> {
    value: T,
    errors: ValidationErrors,
}

impl<T> ValidationBuilder<T> {
    /// Create a new validation builder
    pub fn new(value: T) -> Self {
        Self {
            value,
            errors: ValidationErrors::new(),
        }
    }

    /// Add validation using a function
    pub fn validate<F>(mut self, field: impl Into<String>, f: F) -> Self
    where
        F: FnOnce(&T) -> ValidationResult<()>,
    {
        let field = field.into();
        if let Err(e) = f(&self.value) {
            self.errors.add(field, e.message);
        }
        self
    }

    /// Add conditional validation
    pub fn validate_if<F, P>(
        mut self,
        field: impl Into<String>,
        predicate: P,
        f: F,
    ) -> Self
    where
        P: FnOnce(&T) -> bool,
        F: FnOnce(&T) -> ValidationResult<()>,
    {
        if predicate(&self.value) {
            let field = field.into();
            if let Err(e) = f(&self.value) {
                self.errors.add(field, e.message);
            }
        }
        self
    }

    /// Get the consuming result - (value, errors)
    pub fn build(self) -> (T, ValidationErrors) {
        (self.value, self.errors)
    }

    /// Get result as Result type
    pub fn into_result(self) -> Result<T, ValidationErrors> {
        if self.errors.is_empty() {
            Ok(self.value)
        } else {
            Err(self.errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_validator() {
        let mut validator = RequestValidator::new();

        validator.add_error("email", "invalid");
        assert!(validator.has_errors());
        assert!(validator.into_result().is_err());
    }

    #[test]
    fn test_validation_builder() {
        #[derive(Clone)]
        struct User {
            name: String,
        }

        let user = User {
            name: "John".to_string(),
        };

        let (u, errors) = ValidationBuilder::new(user).build();
        assert_eq!(u.name, "John");
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_if() {
        #[derive(Clone)]
        struct Request {
            is_admin: bool,
        }

        let req = Request { is_admin: true };

        let result = ValidationBuilder::new(req)
            .validate_if("role", |r| r.is_admin, |_| {
                Err(ValidationError::new("role", "admin not allowed"))
            })
            .into_result();

        assert!(result.is_err());
    }
}
