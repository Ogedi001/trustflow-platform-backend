//! Adapters for external libraries
//!
//! This module provides `From` implementations for converting errors from
//! external libraries (adapters) into `AppError`. Each adapter has its own
//! module for better organization.

#[cfg(feature = "redis")]
pub mod redis;

#[cfg(feature = "sqlx")]
pub mod sqlx;

#[cfg(feature = "jwt")]
pub mod jwt;

#[cfg(feature = "reqwest")]
pub mod reqwest;

#[cfg(feature = "validator")]
pub mod validator;

#[cfg(feature = "argon2")]
pub mod argon2;
