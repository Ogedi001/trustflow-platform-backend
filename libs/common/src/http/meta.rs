use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,

    pub timestamp: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub extra: HashMap<String, String>,
}

impl Meta {
    pub fn new() -> Self {
        Self {
            timestamp: OffsetDateTime::now_utc().format(&Rfc3339).unwrap(),
            ..Default::default()
        }
    }

    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }

    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn with_extra(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.extra.insert(key.into(), value.into());
        self
    }
}
