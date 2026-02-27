//!
//! Domain entities for Identity Service
//!
//! Core business entities representing users, profiles, verifications, and sessions.
//! Uses shared value objects from the common library.

pub mod role;
pub mod user;
pub mod verification;
pub mod session;

pub use role::*;
pub use user::*;
pub use verification::*;
pub use session::*;
