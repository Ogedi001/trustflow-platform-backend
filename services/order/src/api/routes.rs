use axum::{
    Router,
    routing::{get, post},
};

use super::handlers::{create_booking, get_booking};

pub fn router() -> Router {
    Router::new()
        .route("/", post(create_booking))
        .route("/{:id}", get(get_booking))
}
