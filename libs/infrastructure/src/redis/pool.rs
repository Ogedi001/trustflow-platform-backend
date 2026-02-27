//! Redis connection pool
//!
//! Provides a pool of Redis connections for high-performance applications.

use crate::redis::error::RedisError;
use futures::Future;
use redis::{Client, aio::ConnectionManager};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

/// Pool of Redis connections
#[derive(Clone)]
pub struct RedisPool {
    client: Arc<Client>,
    manager: Arc<RwLock<Option<ConnectionManager>>>,
}

impl RedisPool {
    /// Create a new Redis pool from connection string
    pub async fn new(connection_string: &str) -> Result<Self, RedisError> {
        info!("Connecting to Redis at {}", connection_string);

        let client =
            Client::open(connection_string).map_err(|e| RedisError::Connection(e.to_string()))?;

        // Test connection by creating a manager
        let mut conn = client
            .get_async_connection()
            .await
            .map_err(|e| RedisError::Connection(e.to_string()))?;

        redis::cmd("PING")
            .query_async::<_, String>(&mut conn)
            .await
            .map_err(|e| RedisError::Connection(e.to_string()))?;

        // Create connection manager for connection pooling
        let manager = ConnectionManager::new(client.clone())
            .await
            .map_err(|e| RedisError::Connection(e.to_string()))?;

        info!("Redis connected successfully with connection pooling");

        Ok(Self {
            client: Arc::new(client),
            manager: Arc::new(RwLock::new(Some(manager))),
        })
    }

    /// Create a new Redis pool from RedisConfig
    pub async fn from_config(config: &crate::redis::RedisConfig) -> Result<Self, RedisError> {
        Self::new(&config.url).await
    }

    /// Get a connection from the pool
    pub async fn get_connection(&self) -> Result<ConnectionManager, RedisError> {
        let guard = self.manager.read().await;
        guard
            .clone()
            .ok_or_else(|| RedisError::Connection("Pool not initialized".to_string()))
    }

    /// Get the underlying client
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Execute a command using the pool
    pub async fn execute<F, T>(&self, f: F) -> Result<T, RedisError>
    where
        F: FnOnce(&mut redis::aio::Connection) -> F,
        F: Future<Output = Result<T, redis::RedisError>>,
    {
        let mut conn = self.get_connection().await?;
        f(&mut conn).await.map_err(RedisError::from)
    }

    /// Close the pool
    pub async fn close(&self) {
        let mut guard = self.manager.write().await;
        if let Some(manager) = guard.take() {
            let _ = manager.close().await;
        }
    }
}

impl From<redis::RedisError> for RedisError {
    fn from(e: redis::RedisError) -> Self {
        match e {
            redis::RedisError::Connection(_) => RedisError::Connection(e.to_string()),
            _ => RedisError::command("redis", e.to_string()),
        }
    }
}
