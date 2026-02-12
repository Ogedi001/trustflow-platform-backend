//! Not found error kind
//!
//! Represents errors when a requested resource cannot be found.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotFoundError {
    pub resource: String,
    pub id: String,
}

impl NotFoundError {
    pub fn new(resource: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            resource: resource.into(),
            id: id.into(),
        }
    }
}

impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} not found: {}", self.resource, self.id)
    }
}
