//! User handlers for Identity Service
//!
//! HTTP handlers for user profile management and account operations.

use axum::{
    extract::{Json, Path, Query, State},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::application::ApplicationContext;
use common::{ApiError, ApiResponse, Pagination};

/// Get current user response
#[derive(Debug, Serialize)]
pub struct GetMeResponse {
    pub id: String,
    pub email: String,
    pub phone: String,
    pub role: String,
    pub status: String,
    pub verification_level: u8,
    pub mfa_enabled: bool,
    pub profile: Option<ProfileResponse>,
    pub created_at: String,
    pub last_login_at: Option<String>,
}

/// Profile response
#[derive(Debug, Serialize)]
pub struct ProfileResponse {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub date_of_birth: Option<String>,
    pub gender: Option<String>,
    pub address: Option<AddressResponse>,
    pub business_name: Option<String>,
    pub business_registration_number: Option<String>,
    pub tax_id: Option<String>,
}

/// Address response
#[derive(Debug, Serialize)]
pub struct AddressResponse {
    pub street: String,
    pub city: String,
    pub state: String,
    pub country: String,
    pub postal_code: Option<String>,
}

/// Update profile request
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfileRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub date_of_birth: Option<String>,
    pub gender: Option<String>,
    pub address: Option<AddressRequest>,
    pub business_name: Option<String>,
    pub business_registration_number: Option<String>,
    pub tax_id: Option<String>,
}

/// Address request
#[derive(Debug, Deserialize, Validate)]
pub struct AddressRequest {
    #[validate(length(min = 1, max = 255))]
    pub street: String,

    #[validate(length(min = 1, max = 100))]
    pub city: String,

    #[validate(length(min = 1, max = 100))]
    pub state: String,

    #[validate(length(min = 2, max = 100))]
    pub country: String,

    #[validate(length(max = 20))]
    pub postal_code: Option<String>,
}

/// Change password request
#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 8))]
    pub old_password: String,

    #[validate(length(min = 8))]
    pub new_password: String,
}

/// Session response
#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub id: String,
    pub device_id: String,
    pub user_agent: String,
    pub ip_address: String,
    pub created_at: String,
    pub last_activity_at: String,
    pub expires_at: String,
}

/// Request deletion request
#[derive(Debug, Deserialize)]
pub struct RequestDeletionRequest {
    pub password: String,
    pub reason: Option<String>,
}

/// Get current user handler
pub async fn get_me(State(_ctx): State<ApplicationContext>) -> ApiResult<impl IntoResponse> {
    // This would fetch the current user from the database

    let response = GetMeResponse {
        id: "placeholder".to_string(),
        email: "placeholder@example.com".to_string(),
        phone: "+2340000000000".to_string(),
        role: "BUYER".to_string(),
        status: "ACTIVE".to_string(),
        verification_level: 0,
        mfa_enabled: false,
        profile: Some(ProfileResponse {
            first_name: None,
            last_name: None,
            display_name: "".to_string(),
            avatar_url: None,
            date_of_birth: None,
            gender: None,
            address: None,
            business_name: None,
            business_registration_number: None,
            tax_id: None,
        }),
        created_at: "2024-01-01T00:00:00Z".to_string(),
        last_login_at: None,
    };

    Ok(ApiResponse::success("User fetched", response))
}

/// Update current user handler
pub async fn update_me(
    State(_ctx): State<ApplicationContext>,
    Json(_req): Json<serde::json::Value>,
) -> ApiResult<impl IntoResponse> {
    // This would update the current user
    Ok(ApiResponse::success_message("User updated successfully"))
}

/// Update profile handler
pub async fn update_profile(
    State(_ctx): State<ApplicationContext>,
    Json(req): Json<UpdateProfileRequest>,
) -> ApiResult<impl IntoResponse> {
    req.validate()?;

    // This would update the user's profile
    Ok(ApiResponse::success_message("Profile updated successfully"))
}

/// Change password handler
pub async fn change_password(
    State(_ctx): State<ApplicationContext>,
    Json(req): Json<ChangePasswordRequest>,
) -> ApiResult<impl IntoResponse> {
    req.validate()?;

    // This would change the user's password
    Ok(ApiResponse::success_message(
        "Password changed successfully",
    ))
}

/// List sessions handler
pub async fn list_sessions(
    State(_ctx): State<ApplicationContext>,
    Query(pagination): Query<Pagination>,
) -> ApiResult<impl IntoResponse> {
    // This would list the user's sessions

    let sessions: Vec<SessionResponse> = vec![];

    Ok(ApiResponse::success("Sessions fetched", sessions))
}

/// Revoke session handler
pub async fn revoke_session(
    State(_ctx): State<ApplicationContext>,
    Path(session_id): Path<String>,
) -> ApiResult<impl IntoResponse> {
    // This would revoke the specified session
    Ok(ApiResponse::success_message("Session revoked successfully"))
}

/// Revoke all sessions handler
pub async fn revoke_all_sessions(
    State(_ctx): State<ApplicationContext>,
) -> ApiResult<impl IntoResponse> {
    // This would revoke all sessions
    Ok(ApiResponse::success_message(
        "All sessions revoked successfully",
    ))
}

/// Request deletion handler
pub async fn request_deletion(
    State(_ctx): State<ApplicationContext>,
    Json(req): Json<RequestDeletionRequest>,
) -> ApiResult<impl IntoResponse> {
    // This would initiate account deletion
    Ok(ApiResponse::success_message(
        "Account deletion request submitted",
    ))
}
