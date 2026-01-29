use super::ApiError;
use axum::response::IntoResponse;

pub async fn not_found_handler() -> impl IntoResponse {
    ApiError::not_found("The requested resource was not found")
        .with_details("Check the API documentation for available endpoints")
}
