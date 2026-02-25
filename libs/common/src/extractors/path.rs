//! Path parameter extractor

use axum::{
    async_trait,
    extract::{FromRequestParts, Path},
    http::request::Parts,
};
use serde::de::DeserializeOwned;

/// Path parameter extractor
pub struct PathExtractor<T>(pub T);

#[async_trait]
impl<T: DeserializeOwned + Send> FromRequestParts<()> for PathExtractor<T> {
    type Rejection = axum::extract::rejection::PathRejection;

    async fn from_request_parts(parts: &mut Parts, _: &()) -> Result<Self, Self::Rejection> {
        match Path::<T>::from_request_parts(parts, &()).await {
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
