//! HTTP converters for mapping domain errors to HTTP responses
//!
//! This module contains converters that map domain error kinds to HTTP API errors.

pub mod auth;
pub mod business;
pub mod database;
pub mod external;
pub mod infrastructure;
pub mod internal;
pub mod not_found;
pub mod validation;
