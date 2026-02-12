//! Internal error kind
//!
//! Represents unknown or internal errors that should not be exposed to users.

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalError {
    pub message: String,
    #[serde(skip)]
    pub source: Option<Arc<dyn Error + Send + Sync>>,
}

impl InternalError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }

    pub fn with_source(message: impl Into<String>, source: Arc<dyn Error + Send + Sync>) -> Self {
        Self {
            message: message.into(),
            source: Some(source),
        }
    }
}

impl PartialEq for InternalError {
    fn eq(&self, other: &Self) -> bool {
        self.message == other.message
    }
}

impl Eq for InternalError {}

impl std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.source {
            Some(source) => write!(f, "Internal error: {} (source: {})", self.message, source),
            None => write!(f, "Internal error: {}", self.message),
        }
    }
}
