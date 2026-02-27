//! API module for Identity Service
//!
//! Contains HTTP handlers, middleware, and route definitions.

pub mod handlers;
pub mod middleware;
pub mod routes;

use crate::application::ApplicationContext;
use axum::extract::State;

pub use handlers::*;
pub use middleware::*;
/// Module re-exports for convenience
pub use routes::router;
