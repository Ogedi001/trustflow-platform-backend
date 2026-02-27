//! API routes for Identity Service
//!
//! Defines all HTTP endpoints for authentication, user management, verification, and admin operations.

use axum::{
    Router,
    routing::{delete, get, patch, post, put},
};
use tower_http::cors::CorsLayer;

use crate::api::handlers::{admin_handler, auth_handler, user_handler, verification_handler};
use crate::api::middleware;
use crate::application::ApplicationContext;

/// Create the main router for Identity Service
pub fn router(app_context: ApplicationContext) -> Router {
    let cors = CorsLayer::permissive();

    Router::new()
        // Health check
        .route("/health", get(handlers::health_check))
        // Auth routes (public)
        .route("/api/v1/auth/register", post(auth_handler::register))
        .route("/api/v1/auth/login", post(auth_handler::login))
        .route("/api/v1/auth/refresh", post(auth_handler::refresh_token))
        .route(
            "/api/v1/auth/forgot-password",
            post(auth_handler::forgot_password),
        )
        .route(
            "/api/v1/auth/reset-password",
            post(auth_handler::reset_password),
        )
        .route(
            "/api/v1/auth/verify-email",
            post(auth_handler::verify_email),
        )
        .route(
            "/api/v1/auth/verify-phone",
            post(auth_handler::verify_phone),
        )
        // MFA routes
        .route("/api/v1/auth/mfa/setup", post(auth_handler::mfa_setup))
        .route("/api/v1/auth/mfa/verify", post(auth_handler::mfa_verify))
        .route("/api/v1/auth/mfa/disable", post(auth_handler::mfa_disable))
        // User routes (authenticated)
        .route("/api/v1/users/me", get(user_handler::get_me))
        .route("/api/v1/users/me", put(user_handler::update_me))
        .route(
            "/api/v1/users/me/profile",
            put(user_handler::update_profile),
        )
        .route(
            "/api/v1/users/me/password",
            post(user_handler::change_password),
        )
        .route(
            "/api/v1/users/me/sessions",
            get(user_handler::list_sessions),
        )
        .route(
            "/api/v1/users/me/sessions/:session_id",
            delete(user_handler::revoke_session),
        )
        .route(
            "/api/v1/users/me/sessions",
            delete(user_handler::revoke_all_sessions),
        )
        .route(
            "/api/v1/users/me/delete",
            post(user_handler::request_deletion),
        )
        // Verification routes (authenticated)
        .route(
            "/api/v1/verification/status",
            get(verification_handler::get_status),
        )
        .route(
            "/api/v1/verification/start",
            post(verification_handler::start_verification),
        )
        .route(
            "/api/v1/verification/upload",
            post(verification_handler::upload_document),
        )
        .route(
            "/api/v1/verification/:id",
            get(verification_handler::get_verification),
        )
        // Admin routes (require admin role)
        .route("/api/v1/admin/users", get(admin_handler::list_users))
        .route("/api/v1/admin/users/:user_id", get(admin_handler::get_user))
        .route(
            "/api/v1/admin/users/:user_id/suspend",
            post(admin_handler::suspend_user),
        )
        .route(
            "/api/v1/admin/users/:user_id/activate",
            post(admin_handler::activate_user),
        )
        .route(
            "/api/v1/admin/users/:user_id/verification",
            put(admin_handler::review_verification),
        )
        .route(
            "/api/v1/admin/users/:user_id/role",
            put(admin_handler::change_role),
        )
        .route(
            "/api/v1/admin/verifications/pending",
            get(admin_handler::list_pending_verifications),
        )
        .route(
            "/api/v1/admin/verifications/:id",
            put(admin_handler::review_verification),
        )
        .route("/api/v1/admin/roles", get(admin_handler::list_roles))
        .route("/api/v1/admin/roles", post(admin_handler::create_role))
        .route(
            "/api/v1/admin/roles/:role_id",
            put(admin_handler::update_role),
        )
        .route(
            "/api/v1/admin/roles/:role_id",
            delete(admin_handler::delete_role),
        )
        .route("/api/v1/admin/stats", get(admin_handler::get_stats))
        // Apply middleware
        .layer(cors)
        .with_state(app_context)
}

/// Module for handlers
pub mod handlers {
    use axum::{Json, response::IntoResponse};
    use common::ApiResponse;
    use serde::Serialize;

    /// Health check handler
    pub async fn health_check() -> impl IntoResponse {
        ApiResponse::success_message("Identity service is healthy")
    }
}

/// Module for handler exports
pub mod handler_modules {
    pub use super::admin_handler;
    pub use super::auth_handler;
    pub use super::user_handler;
    pub use super::verification_handler;
}
