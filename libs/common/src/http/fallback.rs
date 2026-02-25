use super::error::ApiError;
use axum::response::IntoResponse;

/// Default 404 handler
pub async fn handle_404() -> impl IntoResponse {
    ApiError::not_found("The requested resource was not found")
        .with_details("Check the API documentation for available endpoints")
}

/// Alternative name for handle_404
pub async fn not_found_handler() -> impl IntoResponse {
    handle_404().await
}
