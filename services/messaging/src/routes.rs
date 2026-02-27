use axum::{routing::get, Router};
use common::http::response::ApiResponse;

pub fn router() -> Router {
    Router::new().route("/health", get(health))
}

async fn health() -> ApiResponse {
    ApiResponse::success_message("messaging domain is healthy")
}
