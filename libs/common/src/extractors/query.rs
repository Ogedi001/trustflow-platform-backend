//! Query string extractor

use axum::{
    async_trait,
    extract::{FromRequestParts, Query},
    http::request::Parts,
};
use serde::de::DeserializeOwned;

/// Query string extractor
pub struct QueryExtractor<T>(pub T);

#[async_trait]
impl<T: DeserializeOwned + Send> FromRequestParts<()> for QueryExtractor<T> {
    type Rejection = axum::extract::rejection::QueryRejection;

    async fn from_request_parts(parts: &mut Parts, _: &()) -> Result<Self, Self::Rejection> {
        match Query::<T>::from_request_parts(parts, &()).await {
            Ok(Query(value)) => Ok(QueryExtractor(value)),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_extractor_exists() {
        // Compile-time test
        let _: Option<QueryExtractor<serde_json::Value>> = None;
    }
}
