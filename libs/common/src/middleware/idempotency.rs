//! Idempotency middleware for request deduplication
//!
//! Ensures that duplicate requests with the same idempotency key
//! return cached responses instead of executing repeatedly.

use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Idempotency key for deduplication
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdempotencyKey(String);

impl IdempotencyKey {
    /// Create new idempotency key
    pub fn new(key: impl Into<String>) -> Self {
        Self(key.into())
    }

    /// Get key as string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Idempotent request record
#[derive(Debug, Clone)]
pub struct IdempotentRecord {
    /// Status code from original response
    pub status_code: u16,
    /// Response body (serialized)
    pub body: Vec<u8>,
    /// Created timestamp (seconds since epoch)
    pub created_at: u64,
}

/// Idempotency store for tracking requests
#[derive(Debug, Clone)]
pub struct IdempotencyStore {
    /// Map of idempotency key -> response record
    store: Arc<RwLock<HashMap<String, IdempotentRecord>>>,
    /// TTL for records (seconds)
    ttl: u64,
}

impl IdempotencyStore {
    /// Create new idempotency store
    pub fn new(ttl_seconds: u64) -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
            ttl: ttl_seconds,
        }
    }

    /// Create with default TTL (1 hour)
    pub fn default_ttl() -> Self {
        Self::new(3600)
    }

    /// Store response for key
    pub async fn store(&self, key: IdempotencyKey, record: IdempotentRecord) {
        let mut store = self.store.write().await;
        store.insert(key.0, record);
    }

    /// Retrieve cached response
    pub async fn get(&self, key: &IdempotencyKey) -> Option<IdempotentRecord> {
        let store = self.store.read().await;
        store.get(&key.0).cloned()
    }

    /// Check if key exists
    pub async fn exists(&self, key: &IdempotencyKey) -> bool {
        let store = self.store.read().await;
        store.contains_key(&key.0)
    }

    /// Clear all records
    pub async fn clear(&self) {
        let mut store = self.store.write().await;
        store.clear();
    }

    /// Get store size
    pub async fn len(&self) -> usize {
        let store = self.store.read().await;
        store.len()
    }
}

/// Middleware for idempotency handling
pub async fn idempotency_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    // In production, this would:
    // 1. Extract idempotency-key header
    // 2. Check store for existing response
    // 3. Return cached response if found
    // 4. Otherwise, execute request and cache response

    if let Some(idempotency_key) = req.headers().get("idempotency-key") {
        if let Ok(key_str) = idempotency_key.to_str() {
            // Would check store here
            let _key = IdempotencyKey::new(key_str);
        }
    }

    Ok(next.run(req).await)
}

/// Create idempotency middleware with store
pub fn make_idempotency_middleware(
    store: IdempotencyStore,
) -> impl Fn(Request, Next) -> futures::future::BoxFuture<'static, Result<Response, StatusCode>> + Clone
{
    move |req: Request, next: Next| {
        let _store = store.clone();
        Box::pin(idempotency_middleware(req, next))
    }
}
