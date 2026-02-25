//! Request body size limit middleware
//!
//! Enforces maximum request body sizes to prevent memory exhaustion
//! and denial of service attacks.

use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use std::num::NonZeroU64;

/// Body size limit configuration
#[derive(Debug, Clone)]
pub struct BodySizeLimit {
    /// Maximum body size in bytes
    pub max_size: NonZeroU64,
}

impl BodySizeLimit {
    /// Create new body size limit (in bytes)
    pub fn new(max_bytes: u64) -> Option<Self> {
        NonZeroU64::new(max_bytes).map(|max_size| Self { max_size })
    }

    /// Create with 1MB limit (default)
    pub fn default_limit() -> Self {
        Self {
            max_size: NonZeroU64::new(1024 * 1024).unwrap(),
        }
    }

    /// Get limit as bytes
    pub fn bytes(&self) -> u64 {
        self.max_size.get()
    }
}

impl Default for BodySizeLimit {
    fn default() -> Self {
        Self::default_limit()
    }
}

/// Middleware for enforcing body size limits
///
/// # Example
///
/// ```ignore
/// use axum::Router;
/// use common::middleware::BodySizeLimit;
///
/// let limit = BodySizeLimit::new(5 * 1024 * 1024); // 5MB
/// let app = Router::new()
///     .layer(axum::middleware::from_fn(move |req, next| {
///         body_limit_check(req, next, limit.clone())
///     }));
/// ```
pub async fn body_limit_check(
    req: Request,
    next: Next,
    limit: BodySizeLimit,
) -> Result<Response, StatusCode> {
    // In production, this would check actual content-length
    if let Some(content_length_header) = req.headers().get("content-length") {
        if let Ok(content_length_str) = content_length_header.to_str() {
            if let Ok(content_length) = content_length_str.parse::<u64>() {
                if content_length > limit.bytes() {
                    return Err(StatusCode::PAYLOAD_TOO_LARGE);
                }
            }
        }
    }

    Ok(next.run(req).await)
}

/// Create body limit check function with specified limit
pub fn make_body_limit_checker(
    limit: BodySizeLimit,
) -> impl Fn(Request, Next) -> futures::future::BoxFuture<'static, Result<Response, StatusCode>> + Clone
{
    move |req: Request, next: Next| {
        let limit = limit.clone();
        Box::pin(body_limit_check(req, next, limit))
    }
}

/// Standard body size limits
pub mod limits {
    use super::*;

    /// 1MB limit
    pub fn one_mb() -> BodySizeLimit {
        BodySizeLimit::new(1024 * 1024).unwrap()
    }

    /// 5MB limit
    pub fn five_mb() -> BodySizeLimit {
        BodySizeLimit::new(5 * 1024 * 1024).unwrap()
    }

    /// 10MB limit
    pub fn ten_mb() -> BodySizeLimit {
        BodySizeLimit::new(10 * 1024 * 1024).unwrap()
    }

    /// 50MB limit
    pub fn fifty_mb() -> BodySizeLimit {
        BodySizeLimit::new(50 * 1024 * 1024).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_body_size_limit_creation() {
        let limit = BodySizeLimit::new(1024).unwrap();
        assert_eq!(limit.bytes(), 1024);
    }

    #[test]
    fn test_body_size_limit_default() {
        let limit = BodySizeLimit::default();
        assert_eq!(limit.bytes(), 1024 * 1024);
    }

    #[test]
    fn test_body_size_limit_zero_invalid() {
        assert!(BodySizeLimit::new(0).is_none());
    }

    #[test]
    fn test_standard_limits() {
        assert_eq!(limits::one_mb().bytes(), 1024 * 1024);
        assert_eq!(limits::five_mb().bytes(), 5 * 1024 * 1024);
        assert_eq!(limits::ten_mb().bytes(), 10 * 1024 * 1024);
    }
}
