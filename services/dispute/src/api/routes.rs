use axum::{
    Router,
    routing::{delete, get, patch, post, put},
};

pub fn router() -> Router {
    Router::new().route()
}
