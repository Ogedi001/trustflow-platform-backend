//! Domain error kinds
//!
//! This module contains specific error types for different categories of domain errors.
//! Each kind represents a specific type of error that can occur in the application.

pub mod auth;
pub mod business;
pub mod database;
pub mod external;
pub mod infrastructure;
pub mod internal;
pub mod not_found;
pub mod validation;

pub use auth::AuthError;
pub use business::BusinessError;
pub use database::DatabaseError;
pub use external::ExternalServiceError;
pub use infrastructure::InfrastructureError;
pub use internal::InternalError;
pub use not_found::NotFoundError;
pub use validation::ValidationError;
