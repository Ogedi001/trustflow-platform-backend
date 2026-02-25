//! Encoding and serialization utilities

/// JSON utility functions
pub struct JsonUtils;

impl JsonUtils {
    /// Pretty print JSON
    pub fn pretty_print(value: &serde_json::Value) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(value)
    }

    /// Compact JSON (minified)
    pub fn compact(value: &serde_json::Value) -> Result<String, serde_json::Error> {
        serde_json::to_string(value)
    }

    /// Parse JSON from string
    pub fn parse(s: &str) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::from_str(s)
    }

    /// Check if a string is valid JSON
    pub fn is_valid(s: &str) -> bool {
        serde_json::from_str::<serde_json::Value>(s).is_ok()
    }

    /// Merge two JSON objects
    pub fn merge(
        mut a: serde_json::Value,
        b: serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        match (&mut a, &b) {
            (serde_json::Value::Object(obj_a), serde_json::Value::Object(obj_b)) => {
                for (key, value) in obj_b {
                    obj_a.insert(key.clone(), value.clone());
                }
                Ok(a)
            }
            _ => Err("Both values must be JSON objects".to_string()),
        }
    }

    /// Get value at path (simple dot notation)
    pub fn get_path(value: &serde_json::Value, path: &str) -> Option<serde_json::Value> {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = value;

        for part in parts {
            match current {
                serde_json::Value::Object(obj) => {
                    current = obj.get(part)?;
                }
                _ => return None,
            }
        }

        Some(current.clone())
    }
}

/// Base64 encoding utilities
pub struct Base64Utils;

impl Base64Utils {
    /// Encode bytes to base64
    pub fn encode(data: &[u8]) -> String {
        use base64::{engine::general_purpose, Engine as _};
        general_purpose::STANDARD.encode(data)
    }

    /// Decode base64 to bytes
    pub fn decode(s: &str) -> Result<Vec<u8>, base64::DecodeError> {
        use base64::{engine::general_purpose, Engine as _};
        general_purpose::STANDARD.decode(s)
    }

    /// Encode string to base64
    pub fn encode_string(s: &str) -> String {
        use base64::{engine::general_purpose, Engine as _};
        general_purpose::STANDARD.encode(s.as_bytes())
    }

    /// Decode base64 to string
    pub fn decode_string(s: &str) -> Result<String, String> {
        use base64::{engine::general_purpose, Engine as _};
        general_purpose::STANDARD
            .decode(s)
            .map_err(|e| e.to_string())
            .and_then(|bytes| String::from_utf8(bytes).map_err(|e| e.to_string()))
    }

    /// Check if string is valid base64
    pub fn is_valid(s: &str) -> bool {
        use base64::{engine::general_purpose, Engine as _};
        general_purpose::STANDARD.decode(s).is_ok()
    }
}

/// Hex encoding utilities
pub struct HexUtils;

impl HexUtils {
    /// Encode bytes to hex
    pub fn encode(data: &[u8]) -> String {
        hex::encode(data)
    }

    /// Decode hex to bytes
    pub fn decode(s: &str) -> Result<Vec<u8>, hex::FromHexError> {
        hex::decode(s)
    }

    /// Check if string is valid hex
    pub fn is_valid(s: &str) -> bool {
        hex::decode(s).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_parse() {
        let json_str = r#"{"key": "value"}"#;
        assert!(JsonUtils::is_valid(json_str));
        let value = JsonUtils::parse(json_str).unwrap();
        assert_eq!(value["key"], "value");
    }

    #[test]
    fn test_base64() {
        let original = "hello world";
        let encoded = Base64Utils::encode_string(original);
        let decoded = Base64Utils::decode_string(&encoded).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_hex() {
        let data = b"hello";
        let encoded = HexUtils::encode(data);
        let decoded = HexUtils::decode(&encoded).unwrap();
        assert_eq!(decoded, data.to_vec());
    }
}
