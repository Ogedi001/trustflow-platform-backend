//! JSON extractor for request bodies

use axum::{
    Json,
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;

/// Custom JSON extractor with better error messages
pub struct JsonExtractor<T>(pub T);

impl<S, T> FromRequest<S> for JsonExtractor<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = JsonRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<T>::from_request(req, state).await {
            Ok(Json(value)) => Ok(JsonExtractor(value)),
            Err(_) => Err(JsonRejection(StatusCode::BAD_REQUEST, "Invalid JSON")),
        }
    }
}

/// JSON rejection response
pub struct JsonRejection(pub StatusCode, pub &'static str);

impl IntoResponse for JsonRejection {
    fn into_response(self) -> Response {
        (self.0, self.1).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_extractor_exists() {
        // Compile-time test
        let _: Option<JsonExtractor<serde_json::Value>> = None;
    }
}
