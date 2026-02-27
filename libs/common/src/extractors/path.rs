//! Path parameter extractor

use axum::{
    extract::{FromRequestParts, Path},
    http::request::Parts,
};
use serde::de::DeserializeOwned;

/// Path parameter extractor
pub struct PathExtractor<T>(pub T);

impl<S, T> FromRequestParts<S> for PathExtractor<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Send,
{
    type Rejection = axum::extract::rejection::PathRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match Path::<T>::from_request_parts(parts, state).await {
            Ok(Path(value)) => Ok(PathExtractor(value)),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_extractor_exists() {
        // Compile-time test
        let _: Option<PathExtractor<String>> = None;
    }
}
