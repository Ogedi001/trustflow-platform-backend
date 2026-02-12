//! Business logic error kind
//!
//! Represents errors related to business logic violations.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BusinessError {
    Business {
        message: String,
        code: String,
    },
    Conflict {
        message: String,
        #[serde(skip)]
        field: Option<String>,
    },
}

impl BusinessError {
    pub fn business(message: impl Into<String>, code: impl Into<String>) -> Self {
        Self::Business {
            message: message.into(),
            code: code.into(),
        }
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self::Conflict {
            message: message.into(),
            field: None,
        }
    }

    pub fn conflict_with_field(message: impl Into<String>, field: impl Into<String>) -> Self {
        Self::Conflict {
            message: message.into(),
            field: Some(field.into()),
        }
    }
}

impl std::fmt::Display for BusinessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Business { message, code } => {
                write!(f, "Business error: {} (code: {})", message, code)
            }
            Self::Conflict { message, field } => match field {
                Some(field) => write!(f, "Conflict: {} [field: {}]", message, field),
                None => write!(f, "Conflict: {}", message),
            },
        }
    }
}
