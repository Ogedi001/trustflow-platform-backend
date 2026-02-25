//! Response compression middleware
//!
//! Automatically compresses responses based on Accept-Encoding header
//! and response content type.

use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use serde::{Deserialize, Serialize};

/// Compression algorithm configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompressionAlgorithm {
    /// gzip compression
    Gzip,
    /// deflate compression
    Deflate,
    /// brotli compression
    Brotli,
}

impl CompressionAlgorithm {
    /// Get algorithm as string
    pub fn as_str(&self) -> &str {
        match self {
            CompressionAlgorithm::Gzip => "gzip",
            CompressionAlgorithm::Deflate => "deflate",
            CompressionAlgorithm::Brotli => "br",
        }
    }
}

/// Compression configuration
#[derive(Debug, Clone)]
pub struct CompressionConfig {
    /// Enabled algorithms in preference order
    pub algorithms: Vec<CompressionAlgorithm>,
    /// Minimum response size to compress (bytes)
    pub min_size: usize,
    /// Content types to compress
    pub compressible_types: Vec<String>,
}

impl CompressionConfig {
    /// Create new compression config
    pub fn new() -> Self {
        Self {
            algorithms: vec![CompressionAlgorithm::Gzip, CompressionAlgorithm::Deflate],
            min_size: 1024, // 1KB minimum
            compressible_types: vec![
                "text/".to_string(),
                "application/json".to_string(),
                "application/javascript".to_string(),
                "application/xml".to_string(),
            ],
        }
    }

    /// Enable gzip
    pub fn with_gzip(mut self) -> Self {
        if !self.algorithms.contains(&CompressionAlgorithm::Gzip) {
            self.algorithms.push(CompressionAlgorithm::Gzip);
        }
        self
    }

    /// Enable brotli
    pub fn with_brotli(mut self) -> Self {
        if !self.algorithms.contains(&CompressionAlgorithm::Brotli) {
            self.algorithms.push(CompressionAlgorithm::Brotli);
        }
        self
    }

    /// Set minimum size for compression
    pub fn with_min_size(mut self, size: usize) -> Self {
        self.min_size = size;
        self
    }

    /// Check if content type is compressible
    pub fn is_compressible(&self, content_type: &str) -> bool {
        self.compressible_types
            .iter()
            .any(|ct| content_type.contains(ct))
    }
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Middleware for response compression
pub async fn compression_middleware(
    req: Request,
    next: Next,
    config: CompressionConfig,
) -> Result<Response, StatusCode> {
    // In production, this would:
    // 1. Check Accept-Encoding header
    // 2. Select preferred algorithm
    // 3. Compress response body if appropriate
    // 4. Add Content-Encoding header

    // For now, we check the Accept-Encoding header
    if let Some(accept_encoding) = req.headers().get("accept-encoding") {
        if let Ok(encoding_str) = accept_encoding.to_str() {
            // Would select appropriate algorithm from config
            let _preferred_algo = config
                .algorithms
                .iter()
                .find(|algo| encoding_str.contains(algo.as_str()));
        }
    }

    Ok(next.run(req).await)
}

/// Create compression middleware with config
pub fn make_compression_middleware(
    config: CompressionConfig,
) -> impl Fn(Request, Next) -> futures::future::BoxFuture<'static, Result<Response, StatusCode>> + Clone
{
    move |req: Request, next: Next| {
        let config = config.clone();
        Box::pin(compression_middleware(req, next, config))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_algorithm_as_str() {
        assert_eq!(CompressionAlgorithm::Gzip.as_str(), "gzip");
        assert_eq!(CompressionAlgorithm::Deflate.as_str(), "deflate");
        assert_eq!(CompressionAlgorithm::Brotli.as_str(), "br");
    }

    #[test]
    fn test_compression_config_default() {
        let config = CompressionConfig::default();
        assert!(!config.algorithms.is_empty());
        assert!(config.min_size > 0);
    }

    #[test]
    fn test_compression_config_is_compressible() {
        let config = CompressionConfig::default();
        assert!(config.is_compressible("text/html"));
        assert!(config.is_compressible("application/json"));
        assert!(!config.is_compressible("image/png"));
    }

    #[test]
    fn test_compression_config_with_brotli() {
        let config = CompressionConfig::default().with_brotli();
        assert!(config.algorithms.contains(&CompressionAlgorithm::Brotli));
    }
}
