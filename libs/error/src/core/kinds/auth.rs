//! Authentication and authorization error kind
//!
//! Represents errors related to authentication and authorization.

use serde::{Deserialize, Serialize};

use crate::core::AuthErrorCode;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthError {
    Authentication {
        message: String,
        #[serde(skip)]
        code: AuthErrorCode,
    },
    Authorization {
        message: String,
        #[serde(skip)]
        code: AuthErrorCode,
    },
}

impl AuthError {
    pub fn auth(message: impl Into<String>, code: AuthErrorCode) -> Self {
        Self::Authentication {
            message: message.into(),
            code,
        }
    }

    pub fn authz(message: impl Into<String>, code: AuthErrorCode) -> Self {
        Self::Authorization {
            message: message.into(),
            code,
        }
    }
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authentication { message, code } => {
                write!(f, "Authentication failed: {} (code: {:?})", message, code)
            }
            Self::Authorization { message, code } => {
                write!(f, "Authorization failed: {} (code: {:?})", message, code)
            }
        }
    }
}
