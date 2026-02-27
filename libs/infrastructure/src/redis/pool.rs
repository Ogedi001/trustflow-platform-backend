//! Redis client wrapper.

use redis::{Client, aio::MultiplexedConnection};

use crate::redis::{RedisConfig, error::RedisError};

#[derive(Clone)]
pub struct RedisPool {
    client: Client,
}

impl RedisPool {
    pub async fn new(redis_url: &str) -> Result<Self, RedisError> {
        if redis_url.trim().is_empty() {
            return Err(RedisError::Configuration(
                "REDIS_URL cannot be empty".to_string(),
            ));
        }

        let client = Client::open(redis_url).map_err(|e| RedisError::Connection(e.to_string()))?;
        let mut conn = client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| RedisError::Connection(e.to_string()))?;

        redis::cmd("PING")
            .query_async::<String>(&mut conn)
            .await
            .map_err(|e| RedisError::Connection(e.to_string()))?;

        Ok(Self { client })
    }

    pub async fn from_config(config: &RedisConfig) -> Result<Self, RedisError> {
        config.validate().map_err(|e| RedisError::Configuration(e.to_string()))?;
        Self::new(&config.url).await
    }

    pub async fn connection(&self) -> Result<MultiplexedConnection, RedisError> {
        self.client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| RedisError::Connection(e.to_string()))
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
