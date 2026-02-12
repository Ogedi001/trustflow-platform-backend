//! Infrastructure error kind
//!
//! Represents errors related to infrastructure components like cache, queue, storage.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InfrastructureError {
    Infrastructure {
        component: String,
        message: String,
    },
    RateLimit {
        action: String,
        retry_after_seconds: u64,
    },
}

impl InfrastructureError {
    pub fn infrastructure(component: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Infrastructure {
            component: component.into(),
            message: message.into(),
        }
    }

    pub fn rate_limit(action: impl Into<String>, retry_after: u64) -> Self {
        Self::RateLimit {
            action: action.into(),
            retry_after_seconds: retry_after,
        }
    }
}

impl std::fmt::Display for InfrastructureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Infrastructure { component, message } => {
                write!(f, "{}: {}", component, message)
            }
            Self::RateLimit {
                action,
                retry_after_seconds,
            } => {
                write!(
                    f,
                    "Rate limit exceeded for '{}', retry after {} seconds",
                    action, retry_after_seconds
                )
            }
        }
    }
}
