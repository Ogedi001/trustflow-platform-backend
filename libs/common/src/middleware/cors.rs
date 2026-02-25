//! Cross-Origin Resource Sharing (CORS) middleware
//!
//! Handles CORS policy enforcement and preflight requests.

use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use std::collections::HashSet;

/// CORS policy configuration
#[derive(Debug, Clone)]
pub struct CorsPolicy {
    /// Allowed origins
    pub allowed_origins: HashSet<String>,
    /// Allow all origins
    pub allow_all: bool,
    /// Allowed HTTP methods
    pub allowed_methods: HashSet<String>,
    /// Allowed headers
    pub allowed_headers: HashSet<String>,
    /// Maximum age for preflight cache (seconds)
    pub max_age: u64,
    /// Allow credentials
    pub allow_credentials: bool,
}

impl CorsPolicy {
    /// Create new CORS policy
    pub fn new() -> Self {
        Self {
            allowed_origins: HashSet::new(),
            allow_all: false,
            allowed_methods: ["GET", "POST", "PUT", "DELETE", "OPTIONS", "PATCH"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            allowed_headers: ["Content-Type", "Authorization"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            max_age: 3600,
            allow_credentials: false,
        }
    }

    /// Allow all origins
    pub fn allow_all_origins(mut self) -> Self {
        self.allow_all = true;
        self
    }

    /// Add allowed origin
    pub fn add_origin(mut self, origin: impl Into<String>) -> Self {
        self.allowed_origins.insert(origin.into());
        self
    }

    /// Add allowed origins
    pub fn with_origins(mut self, origins: Vec<impl Into<String>>) -> Self {
        for origin in origins {
            self.allowed_origins.insert(origin.into());
        }
        self
    }

    /// Add allowed method
    pub fn add_method(mut self, method: impl Into<String>) -> Self {
        self.allowed_methods.insert(method.into());
        self
    }

    /// Add allowed header
    pub fn add_header(mut self, header: impl Into<String>) -> Self {
        self.allowed_headers.insert(header.into());
        self
    }

    /// Set max age for preflight cache
    pub fn with_max_age(mut self, seconds: u64) -> Self {
        self.max_age = seconds;
        self
    }

    /// Allow credentials
    pub fn with_credentials(mut self) -> Self {
        self.allow_credentials = true;
        self
    }

    /// Check if origin is allowed
    pub fn is_origin_allowed(&self, origin: &str) -> bool {
        self.allow_all || self.allowed_origins.contains(origin)
    }

    /// Check if method is allowed
    pub fn is_method_allowed(&self, method: &str) -> bool {
        self.allowed_methods.contains(method)
    }

    /// Get allowed methods as comma-separated string
    pub fn allowed_methods_str(&self) -> String {
        let mut methods: Vec<_> = self.allowed_methods.iter().cloned().collect();
        methods.sort();
        methods.join(", ")
    }

    /// Get allowed headers as comma-separated string
    pub fn allowed_headers_str(&self) -> String {
        let mut headers: Vec<_> = self.allowed_headers.iter().cloned().collect();
        headers.sort();
        headers.join(", ")
    }
}

impl Default for CorsPolicy {
    fn default() -> Self {
        Self::new()
    }
}

/// Middleware for CORS handling
pub async fn cors_middleware(
    req: Request,
    next: Next,
    policy: CorsPolicy,
) -> Result<Response, StatusCode> {
    // Check for preflight request
    if req.method() == axum::http::Method::OPTIONS {
        // Check origin
        if let Some(origin) = req.headers().get("origin") {
            if let Ok(origin_str) = origin.to_str() {
                if !policy.is_origin_allowed(origin_str) {
                    return Err(StatusCode::FORBIDDEN);
                }
            }
        }
    }

    Ok(next.run(req).await)
}

/// Create CORS middleware with policy
pub fn make_cors_middleware(
    policy: CorsPolicy,
) -> impl Fn(Request, Next) -> futures::future::BoxFuture<'static, Result<Response, StatusCode>> + Clone
{
    move |req: Request, next: Next| {
        let policy = policy.clone();
        Box::pin(cors_middleware(req, next, policy))
    }
}

/// Preset CORS policies
pub mod presets {
    use super::*;

    /// Policy that allows all origins
    pub fn permissive() -> CorsPolicy {
        CorsPolicy::new().allow_all_origins()
    }

    /// Policy for development
    pub fn development() -> CorsPolicy {
        CorsPolicy::new()
            .add_origin("http://localhost:3000")
            .add_origin("http://localhost:3001")
            .add_origin("http://localhost:8080")
            .with_credentials()
    }

    /// Restricted policy (only specific origins)
    pub fn restricted(origins: Vec<&str>) -> CorsPolicy {
        let mut policy = CorsPolicy::new();
        for origin in origins {
            policy = policy.add_origin(origin);
        }
        policy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cors_policy_creation() {
        let policy = CorsPolicy::new();
        assert!(!policy.allow_all);
        assert!(!policy.allowed_origins.is_empty());
    }

    #[test]
    fn test_cors_policy_allow_all_origins() {
        let policy = CorsPolicy::new().allow_all_origins();
        assert!(policy.is_origin_allowed("*"));
        assert!(policy.is_origin_allowed("http://example.com"));
    }

    #[test]
    fn test_cors_policy_add_origin() {
        let policy = CorsPolicy::new().add_origin("http://example.com");
        assert!(policy.is_origin_allowed("http://example.com"));
        assert!(!policy.is_origin_allowed("http://other.com"));
    }

    #[test]
    fn test_cors_policy_is_method_allowed() {
        let policy = CorsPolicy::new();
        assert!(policy.is_method_allowed("GET"));
        assert!(policy.is_method_allowed("POST"));
    }

    #[test]
    fn test_cors_preset_development() {
        let policy = presets::development();
        assert!(policy.is_origin_allowed("http://localhost:3000"));
        assert!(policy.allow_credentials);
    }
}
