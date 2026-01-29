use crate::api;
use axum::Router;
use common::http::{self as http_common};

pub fn build_app() -> Router {
    let api_router = api::router();
    Router::new()
        .nest("/api/orders", api_router)
        .fallback(http_common::fallback::not_found_handler)
}
