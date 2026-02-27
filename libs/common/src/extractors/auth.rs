//! Authentication extractor

use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};

/// Bearer token extractor
pub struct BearerToken(pub String);

impl<S> FromRequestParts<S> for BearerToken
where
    S: Send + Sync,
{
    type Rejection = AuthorityRejection;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or(AuthorityRejection(
                StatusCode::UNAUTHORIZED,
                "Missing authorization header",
            ))?;

        let token = header
            .strip_prefix("Bearer ")
            .or_else(|| header.strip_prefix("bearer "))
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .ok_or(AuthorityRejection(
                StatusCode::UNAUTHORIZED,
                "Missing or invalid authorization header",
            ))?;

        Ok(BearerToken(token.to_string()))
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
