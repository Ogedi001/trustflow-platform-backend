//! Redis Pub/Sub for inter-service communication
//!
//! Provides Redis pub/sub functionality for event-driven architectures,
//! real-time notifications, and distributed messaging.
//!
//! ## Feature Flags
//!
//! - `redis`: Enables Redis support (enabled by default with `full` feature)

#[cfg(feature = "redis")]
use async_trait::async_trait;

#[cfg(feature = "redis")]
use serde::{Serialize, de::DeserializeOwned};

#[cfg(feature = "redis")]
use super::{RedisError, RedisPool};
#[cfg(feature = "redis")]
use crate::redis::key::RedisKey;

#[cfg(feature = "redis")]
use futures_util::stream::StreamExt;
#[cfg(feature = "redis")]
use std::sync::Arc;
#[cfg(feature = "redis")]
use tokio::sync::broadcast;

/// Message wrapper for pub/sub messages
#[cfg(feature = "redis")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubSubMessage {
    /// Topic/channel name
    pub topic: String,
    /// Message payload (JSON serialized)
    pub payload: String,
    /// Optional message ID for deduplication
    pub message_id: Option<String>,
    /// Timestamp when message was published
    pub timestamp: i64,
}

/// Pub/Sub trait for message publishing and subscribing
#[cfg(feature = "redis")]
#[async_trait]
pub trait PubSub: Send + Sync {
    /// Publish a message to a channel
    async fn publish(&self, channel: &str, message: &str) -> Result<u64, RedisError>;

    /// Subscribe to a channel
    async fn subscribe(&self, channel: &str) -> Result<Subscription, RedisError>;

    /// Publish a serialized message to a channel
    async fn publish_json<T: Serialize>(
        &self,
        channel: &str,
        message: &T,
    ) -> Result<u64, RedisError>;
}

/// Subscription handle for receiving messages
#[cfg(feature = "redis")]
pub struct Subscription {
    pub channel: String,
    pub receiver: broadcast::Receiver<String>,
}

/// Redis Pub/Sub implementation
#[cfg(feature = "redis")]
#[derive(Clone)]
pub struct RedisPubSub {
    pool: RedisPool,
    prefix: String,
    sender: Arc<broadcast::Sender<String>>,
}

#[cfg(feature = "redis")]
impl RedisPubSub {
    /// Create a new Redis pub/sub instance
    pub fn new(pool: RedisPool, prefix: impl Into<String>) -> Self {
        let (sender, _) = broadcast::channel(1000);
        Self {
            pool,
            prefix: prefix.into(),
            sender: Arc::new(sender),
        }
    }

    /// Get the channel prefix
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// Get prefixed channel name
    fn channel_name(&self, channel: &str) -> RedisKey {
        RedisKey::from_parts([&self.prefix, "pubsub", channel])
    }

    /// Subscribe to a channel and return a stream of messages
    pub async fn subscribe_to_channel(
        &self,
        channel: &str,
    ) -> Result<broadcast::Receiver<String>, RedisError> {
        let conn = self.pool.get_connection().await?;

        let channel_name = self.channel_name(channel).as_str();

        // Subscribe to Redis channel
        let mut pubsub = redis::cmd("SUBSCRIBE")
            .arg(channel_name)
            .query_async::<_, ()>(conn)
            .await
            .map_err(|e| RedisError::command("subscribe", e.to_string()))?;

        let sender = self.sender.clone();

        // Spawn a task to forward Redis messages to the broadcast channel
        tokio::spawn(async move {
            while let Some(msg) = pubsub.on_message().next().await {
                let payload: String = msg.get_payload().unwrap_or_default();
                let _ = sender.send(payload);
            }
        });

        Ok(self.sender.subscribe())
    }
}

#[cfg(feature = "redis")]
#[async_trait]
impl PubSub for RedisPubSub {
    async fn publish(&self, channel: &str, message: &str) -> Result<u64, RedisError> {
        let conn = self.pool.get_connection().await?;
        let channel_name = self.channel_name(channel).as_str();

        let result: u64 = redis::cmd("PUBLISH")
            .arg(channel_name)
            .arg(message)
            .query_async(conn)
            .await
            .map_err(|e| RedisError::command("publish", e.to_string()))?;

        Ok(result)
    }

    async fn subscribe(&self, channel: &str) -> Result<Subscription, RedisError> {
        let receiver = self.subscribe_to_channel(channel).await?;
        Ok(Subscription {
            channel: channel.to_string(),
            receiver,
        })
    }

    async fn publish_json<T: Serialize>(
        &self,
        channel: &str,
        message: &T,
    ) -> Result<u64, RedisError> {
        let payload = serde_json::to_string(message)
            .map_err(|e| RedisError::serialization("JSON", e.to_string()))?;
        self.publish(channel, &payload).await
    }
}

/// Builder for creating Pub/Sub messages
#[cfg(feature = "redis")]
pub struct MessageBuilder {
    topic: String,
    payload: String,
    message_id: Option<String>,
    timestamp: i64,
}

#[cfg(feature = "redis")]
impl MessageBuilder {
    /// Create a new message builder
    pub fn new(topic: impl Into<String>, payload: impl Into<String>) -> Self {
        Self {
            topic: topic.into(),
            payload: payload.into(),
            message_id: None,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    /// Set message ID for deduplication
    pub fn with_message_id(mut self, id: impl Into<String>) -> Self {
        self.message_id = Some(id.into());
        self
    }

    /// Build the message
    pub fn build(self) -> PubSubMessage {
        PubSubMessage {
            topic: self.topic,
            payload: self.payload,
            message_id: self.message_id,
            timestamp: self.timestamp,
        }
    }
}

/// Helper to create a new message
#[cfg(feature = "redis")]
pub fn message(topic: impl Into<String>, payload: impl Into<String>) -> MessageBuilder {
    MessageBuilder::new(topic, payload)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_builder_creates_message() {
        let msg = message("test-topic", "hello world")
            .with_message_id("msg-123")
            .build();

        assert_eq!(msg.topic, "test-topic");
        assert_eq!(msg.payload, "hello world");
        assert_eq!(msg.message_id, Some("msg-123".to_string()));
    }

    #[test]
    fn message_builder_without_id() {
        let msg = message("test-topic", "hello world").build();

        assert_eq!(msg.topic, "test-topic");
        assert_eq!(msg.payload, "hello world");
        assert_eq!(msg.message_id, None);
    }
}
