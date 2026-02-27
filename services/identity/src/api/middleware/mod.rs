//! Middleware for Identity Service
//!
//! Re-exports and extends common middleware with identity-specific functionality.

pub use common::middleware::{
    auth_middleware, cors_layer, logging_middleware, rate_limit_middleware, request_id_middleware,
    require_role, AuthState, CorsConfig, CurrentUser, CurrentUserExt, JwtClaims, JwtService,
    KeyExtractor, LoggingState, RateLimitState, TimeoutConfig,
};

pub use common::middleware::RateLimiter as MiddlewareRateLimiter;

use axum::body::Body;
use axum::extract::Request;
use common::CurrentUser as CommonCurrentUser;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::{Layer, Service};

/// Role hierarchy check - identity specific
/// Checks if user role meets the required role level
pub fn has_required_role(user_role: &str, required_role: &str) -> bool {
    let hierarchy = [
        "SUPERADMIN",
        "ADMIN",
        "MODERATOR",
        "SELLER",
        "BUYER",
        "GUEST",
    ];

    let user_idx = hierarchy
        .iter()
        .position(|r| r == &user_role.to_uppercase());
    let required_idx = hierarchy
        .iter()
        .position(|r| r == &required_role.to_uppercase());

    match (user_idx, required_idx) {
        (Some(u), Some(r)) => u <= r, // Higher privilege = lower index
        _ => false,
    }
}

/// User ID extractor from CurrentUser
pub fn extract_user_id(current_user: &CommonCurrentUser) -> String {
    current_user.user_id.to_string()
}

/// Create auth state for identity service
pub fn create_auth_state(
    jwt_secret: &str,
    jwt_issuer: &str,
    jwt_audience: &str,
    redis_pool: infrastructure::redis::RedisPool,
) -> AuthState {
    AuthState {
        jwt_service: JwtService::new(
            jwt_secret,
            jwt_issuer,
            jwt_audience,
            3600,   // access token expiry: 1 hour
            604800, // refresh token expiry: 7 days
        ),
        redis: redis_pool,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_hierarchy() {
        assert!(has_required_role("SUPERADMIN", "ADMIN"));
        assert!(has_required_role("ADMIN", "SELLER"));
        assert!(has_required_role("SELLER", "BUYER"));
        assert!(has_required_role("BUYER", "GUEST"));

        // Same role should work
        assert!(has_required_role("ADMIN", "ADMIN"));

        // Reverse should not work
        assert!(!has_required_role("BUYER", "ADMIN"));
        assert!(!has_required_role("SELLER", "SUPERADMIN"));
    }
}
