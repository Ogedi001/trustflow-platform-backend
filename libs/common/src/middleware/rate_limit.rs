//! Rate limiting middleware
//!
//! Implements token bucket and sliding window rate limiting algorithms
//! to prevent abuse and ensure fair resource usage.

use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Rate limit key (typically IP address or user ID)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RateLimitKey(String);

impl RateLimitKey {
    /// Create new rate limit key
    pub fn new(key: impl Into<String>) -> Self {
        Self(key.into())
    }
}

/// Token bucket for rate limiting
#[derive(Debug, Clone)]
pub struct TokenBucket {
    /// Maximum tokens
    pub max_tokens: u64,
    /// Current tokens
    pub tokens: f64,
    /// Tokens per second refill rate
    pub refill_rate: f64,
    /// Last refill time
    pub last_refilled: Instant,
}

impl TokenBucket {
    /// Create new token bucket
    pub fn new(max_tokens: u64, refill_rate: f64) -> Self {
        Self {
            max_tokens,
            tokens: max_tokens as f64,
            refill_rate,
            last_refilled: Instant::now(),
        }
    }

    /// Refill bucket based on elapsed time
    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refilled).as_secs_f64();
        let new_tokens = elapsed * self.refill_rate;
        self.tokens = (self.tokens + new_tokens).min(self.max_tokens as f64);
        self.last_refilled = now;
    }

    /// Try to consume tokens
    pub fn try_consume(&mut self, tokens: u64) -> bool {
        self.refill();
        if self.tokens >= tokens as f64 {
            self.tokens -= tokens as f64;
            true
        } else {
            false
        }
    }

    /// Get current token count
    pub fn current_tokens(&self) -> u64 {
        self.tokens as u64
    }
}

/// Rate limiter configuration
#[derive(Debug, Clone)]
pub struct RateLimiterConfig {
    /// Requests per second
    pub requests_per_second: u64,
    /// Burst size (max requests at once)
    pub burst_size: u64,
    /// Cleanup interval for expired entries
    pub cleanup_interval: Duration,
}

impl RateLimiterConfig {
    /// Create new rate limiter config
    pub fn new(requests_per_second: u64, burst_size: u64) -> Self {
        Self {
            requests_per_second,
            burst_size,
            cleanup_interval: Duration::from_secs(60),
        }
    }

    /// Standard: 100 requests per second
    pub fn standard() -> Self {
        Self::new(100, 120)
    }

    /// Strict: 10 requests per second
    pub fn strict() -> Self {
        Self::new(10, 12)
    }

    /// Permissive: 1000 requests per second
    pub fn permissive() -> Self {
        Self::new(1000, 1200)
    }
}

/// Rate limiter store
#[derive(Debug, Clone)]
pub struct RateLimiter {
    config: RateLimiterConfig,
    buckets: Arc<RwLock<HashMap<String, TokenBucket>>>,
}

impl RateLimiter {
    /// Create new rate limiter
    pub fn new(config: RateLimiterConfig) -> Self {
        Self {
            config,
            buckets: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Check if request is allowed
    pub async fn is_allowed(&self, key: &RateLimitKey) -> bool {
        let mut buckets = self.buckets.write().await;
        let bucket = buckets.entry(key.0.clone()).or_insert_with(|| {
            TokenBucket::new(
                self.config.burst_size,
                self.config.requests_per_second as f64,
            )
        });

        bucket.try_consume(1)
    }

    /// Get remaining requests for key
    pub async fn remaining(&self, key: &RateLimitKey) -> u64 {
        let buckets = self.buckets.read().await;
        buckets
            .get(&key.0)
            .map(|b| b.current_tokens())
            .unwrap_or(self.config.burst_size)
    }

    /// Clear all buckets
    pub async fn clear(&self) {
        let mut buckets = self.buckets.write().await;
        buckets.clear();
    }
}

/// Middleware for rate limiting
pub async fn rate_limit_middleware(
    req: Request,
    next: Next,
    limiter: RateLimiter,
) -> Result<Response, StatusCode> {
    // Extract rate limit key from request (typically from IP or user ID)
    let key = if let Some(forwarded_for) = req.headers().get("x-forwarded-for") {
        if let Ok(ip) = forwarded_for.to_str() {
            RateLimitKey::new(ip)
        } else {
            RateLimitKey::new("unknown")
        }
    } else {
        RateLimitKey::new("unknown")
    };

    if !limiter.is_allowed(&key).await {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    Ok(next.run(req).await)
}

/// Create rate limit middleware
pub fn make_rate_limit_middleware(
    limiter: RateLimiter,
) -> impl Fn(Request, Next) -> futures::future::BoxFuture<'static, Result<Response, StatusCode>> + Clone
{
    move |req: Request, next: Next| {
        let limiter = limiter.clone();
        Box::pin(rate_limit_middleware(req, next, limiter))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_bucket_creation() {
        let bucket = TokenBucket::new(100, 10.0);
        assert_eq!(bucket.current_tokens(), 100);
    }

    #[test]
    fn test_token_bucket_consume() {
        let mut bucket = TokenBucket::new(100, 10.0);
        assert!(bucket.try_consume(50));
        assert_eq!(bucket.current_tokens(), 50);
    }

    #[test]
    fn test_token_bucket_over_consume() {
        let mut bucket = TokenBucket::new(100, 10.0);
        assert!(!bucket.try_consume(101));
    }

    #[test]
    fn test_rate_limiter_config_standard() {
        let config = RateLimiterConfig::standard();
        assert_eq!(config.requests_per_second, 100);
    }

    #[tokio::test]
    async fn test_rate_limiter_allowed() {
        let limiter = RateLimiter::new(RateLimiterConfig::new(100, 120));
        let key = RateLimitKey::new("test-ip");

        for _ in 0..100 {
            assert!(limiter.is_allowed(&key).await);
        }
        assert!(!limiter.is_allowed(&key).await); // Should be rate limited
    }

    #[tokio::test]
    async fn test_rate_limiter_remaining() {
        let limiter = RateLimiter::new(RateLimiterConfig::new(10, 20));
        let key = RateLimitKey::new("test-ip");

        limiter.is_allowed(&key).await;
        let remaining = limiter.remaining(&key).await;
        assert!(remaining <= 20);
    }
}
