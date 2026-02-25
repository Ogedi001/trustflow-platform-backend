//! JSON serialization helpers

use serde::Serialize;

/// JSON field builder for fluent JSON construction
pub struct JsonBuilder {
    value: serde_json::Value,
}

impl JsonBuilder {
    /// Create a new JSON object builder
    pub fn new() -> Self {
        Self {
            value: serde_json::json!({}),
        }
    }

    /// Add a field to the JSON object
    pub fn field(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        if let serde_json::Value::Object(ref mut obj) = self.value {
            if let Ok(json_value) = serde_json::to_value(value) {
                obj.insert(key.into(), json_value);
            }
        }
        self
    }

    /// Add a field only if condition is true
    pub fn field_if(self, condition: bool, key: impl Into<String>, value: impl Serialize) -> Self {
        if condition {
            self.field(key, value)
        } else {
            self
        }
    }

    /// Add an object field
    pub fn object(mut self, key: impl Into<String>, builder: JsonBuilder) -> Self {
        if let serde_json::Value::Object(ref mut obj) = self.value {
            obj.insert(key.into(), builder.value);
        }
        self
    }

    /// Add an array of items
    pub fn array(mut self, key: impl Into<String>, items: Vec<serde_json::Value>) -> Self {
        if let serde_json::Value::Object(ref mut obj) = self.value {
            obj.insert(key.into(), serde_json::json!(items));
        }
        self
    }

    /// Build the final JSON value
    pub fn build(self) -> serde_json::Value {
        self.value
    }

    /// Build and serialize to string
    pub fn to_string_pretty(self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.value)
    }
}

impl Default for JsonBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// JSON response builder for API responses
pub struct JsonResponse {
    builder: JsonBuilder,
}

impl JsonResponse {
    /// Create a new response builder
    pub fn new() -> Self {
        Self {
            builder: JsonBuilder::new(),
        }
    }

    /// Set success status
    pub fn success(mut self, msg: impl Into<String>) -> Self {
        self.builder = self
            .builder
            .field("success", true)
            .field("message", msg.into());
        self
    }

    /// Set error status
    pub fn error(mut self, msg: impl Into<String>) -> Self {
        self.builder = self
            .builder
            .field("success", false)
            .field("error", msg.into());
        self
    }

    /// Add data to response
    pub fn data(mut self, value: impl Serialize) -> Self {
        self.builder = self.builder.field("data", value);
        self
    }

    /// Add metadata
    pub fn meta(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        self.builder = self.builder.field(key.into(), value);
        self
    }

    /// Build the response
    pub fn build(self) -> serde_json::Value {
        self.builder.build()
    }
}

impl Default for JsonResponse {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_json_builder() {
        let json = JsonBuilder::new()
            .field("name", "John")
            .field("age", 30)
            .build();

        assert_eq!(json["name"], "John");
        assert_eq!(json["age"], 30);
    }

    #[test]
    fn test_json_builder_nested() {
        let user = JsonBuilder::new()
            .field("name", "John")
            .field("email", "john@example.com");

        let root = JsonBuilder::new().object("user", user).build();

        assert_eq!(root["user"]["name"], "John");
    }

    #[test]
    fn test_json_response() {
        let response = JsonResponse::new()
            .success("User created")
            .data(json!({"id": 123}))
            .build();

        assert_eq!(response["success"], true);
        assert_eq!(response["data"]["id"], 123);
    }

    #[test]
    fn test_field_if() {
        let json = JsonBuilder::new()
            .field_if(true, "included", "yes")
            .field_if(false, "excluded", "no")
            .build();

        assert!(json["included"].is_string());
        assert!(!json["excluded"].is_string());
    }
}
