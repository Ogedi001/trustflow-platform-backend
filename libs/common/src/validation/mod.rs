//! Validation module
//!
//! This module provides reusable validation rules, patterns, and utilities
//! for validating requests, domain objects, and common constraints.
//!
//! ## Organization
//!
//! - `rules` - Basic validation rules (StringRules, EmailRules, PhoneRules, etc.)
//! - `request` - Request-level validation traits and builders
//!
//! ## Quick Start
//!
//! ```rust
//! use common::validation::{Validate, ValidationErrors};
//! use common::validation::rules::{StringRules, ValidationResult};
//!
//! fn validate_username(name: &str) -> ValidationResult<()> {
//!     StringRules::not_empty(name, "username")?;
//!     StringRules::length_range(name, 3, 50, "username")?;
//!     Ok(())
//! }
//! ```

pub mod request;
pub mod rules;

pub use request::{RequestValidator, Validate, ValidateWith, ValidationBuilder};
pub use rules::{
    EmailRules, NumberRules, PhoneRules, StringRules, ValidationError, ValidationErrors,
    ValidationResult,
};

/// Prelude for validation module
pub mod prelude {
    //! Import common validation items with `use common::validation::prelude::*;`
    pub use super::rules::*;
    pub use super::{
        EmailRules, NumberRules, PhoneRules, RequestValidator, StringRules, Validate,
        ValidateWith, ValidationBuilder, ValidationErrors, ValidationResult,
    };
}
