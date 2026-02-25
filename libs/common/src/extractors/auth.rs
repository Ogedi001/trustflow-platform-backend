//! Authentication extractor

use axum::{
    async_trait,
    extract::{FromRequestParts, TypedHeader},
    headers::Authorization,
    headers::authorization::Bearer,
    http::{request::Parts, StatusCode},
};

/// Bearer token extractor
pub struct BearerToken(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for BearerToken
where
    S: Send + Sync,
{
    type Rejection = AuthorityRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state).await {
            Ok(TypedHeader(Authorization(bearer))) => {
                Ok(BearerToken(bearer.0.to_string()))
            }
            Err(_) => Err(AuthorityRejection(
                StatusCode::UNAUTHORIZED,
                "Missing or invalid authorization header",
            )),
        }
    }
}

/// Authentication rejection
pub struct AuthorityRejection(pub StatusCode, pub &'static str);

impl axum::response::IntoResponse for AuthorityRejection {
    fn into_response(self) -> axum::response::Response {
        (self.0, self.1).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bearer_token_exists() {
        // Compile-time test
        let _: Option<BearerToken> = None;
    }
}
