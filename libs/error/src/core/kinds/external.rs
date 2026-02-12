//! External service error kind
//!
//! Represents errors that occur when interacting with external services.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalServiceError {
    pub service: String,
    pub message: String,
    #[serde(skip)]
    pub status_code: Option<u16>,
}

impl ExternalServiceError {
    pub fn new(service: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            service: service.into(),
            message: message.into(),
            status_code: None,
        }
    }

    pub fn with_status(
        service: impl Into<String>,
        message: impl Into<String>,
        status_code: u16,
    ) -> Self {
        Self {
            service: service.into(),
            message: message.into(),
            status_code: Some(status_code),
        }
    }
}

impl fmt::Display for ExternalServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.service, self.message)
    }
}
