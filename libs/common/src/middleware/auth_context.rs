//! Authentication context middleware
//!
//! Extracts authentication context from bearer tokens and inserts it into
//! request extensions for use in handlers.

use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use axum::{extract::Request, middleware};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Authentication context extracted from bearer token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthContext {
    /// User ID from token
    pub user_id: String,
    /// Subject (typically user identifier)
    pub subject: Option<String>,
    /// Token scopes
    pub scopes: Vec<String>,
    /// Token issuer
    pub issuer: Option<String>,
}

impl AuthContext {
    /// Create new auth context
    pub fn new(user_id: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into(),
            subject: None,
            scopes: Vec::new(),
            issuer: None,
        }
    }

    /// Set subject
    pub fn with_subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    /// Add scope
    pub fn with_scope(mut self, scope: impl Into<String>) -> Self {
        self.scopes.push(scope.into());
        self
    }

    /// Set issuer
    pub fn with_issuer(mut self, issuer: impl Into<String>) -> Self {
        self.issuer = Some(issuer.into());
        self
    }

    /// Check if context has a specific scope
    pub fn has_scope(&self, scope: &str) -> bool {
        self.scopes.iter().any(|s| s == scope)
    }
}

/// Middleware for handling authentication context
pub async fn auth_context(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    // In a real implementation, this would:
    // 1. Extract bearer token from Authorization header
    // 2. Validate and decode JWT
    // 3. Create AuthContext from token claims
    // 4. Insert into request extensions

    // For now, provide a basic implementation that checks for Authorization header
    let headers = req.headers();
    if let Some(auth_header) = headers.get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                // In production, would validate JWT here.
                let context = AuthContext::new("user-from-token").with_subject("user-subject");
                req.extensions_mut().insert(Arc::new(context));
            }
        }
    }

    Ok(next.run(req).await)
}

/// Layer for auth context middleware
pub fn auth_context_layer() -> impl Clone {
    middleware::from_fn::<_, ()>(|req, next| Box::pin(auth_context(req, next)))
}
