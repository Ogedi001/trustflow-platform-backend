//! Serde serialization utilities

use serde::{Deserialize, Serialize};

/// Helper for optional serialization fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionalField<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<T>,
}

impl<T> OptionalField<T> {
    /// Create with some value
    pub fn some(value: T) -> Self {
        Self { value: Some(value) }
    }

    /// Create with no value
    pub fn none() -> Self {
        Self { value: None }
    }

    /// Check if present
    pub fn is_some(&self) -> bool {
        self.value.is_some()
    }

    /// Check if none
    pub fn is_none(&self) -> bool {
        self.value.is_none()
    }

    /// Get the inner value
    pub fn as_ref(&self) -> Option<&T> {
        self.value.as_ref()
    }

    /// Consume and get the inner value
    pub fn into_inner(self) -> Option<T> {
        self.value
    }
}

impl<T: Default> Default for OptionalField<T> {
    fn default() -> Self {
        Self { value: None }
    }
}

/// Serialize/deserialize wrapper for compact representation
pub mod compact {
    use serde::{Deserialize, Deserializer, Serialize, Serializer, de::DeserializeOwned};

    /// Serialize to compact string representation
    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
        S: Serializer,
    {
        let json = serde_json::to_string(value).map_err(serde::ser::Error::custom)?;
        json.serialize(serializer)
    }

    /// Deserialize from compact string representation
    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: DeserializeOwned,
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        serde_json::from_str(&s).map_err(serde::de::Error::custom)
    }
}

/// Rename rules for different serialization styles
pub mod rename {
    /// Convert to snake_case
    pub fn to_snake_case(s: &str) -> String {
        let mut result = String::new();
        for (i, c) in s.chars().enumerate() {
            if c.is_uppercase() {
                if i > 0 {
                    result.push('_');
                }
                result.push(c.to_lowercase().next().unwrap_or(c));
            } else {
                result.push(c);
            }
        }
        result
    }

    /// Convert to camelCase
    pub fn to_camel_case(s: &str) -> String {
        let parts: Vec<&str> = s.split('_').collect();
        let mut result = parts[0].to_string();

        for part in &parts[1..] {
            if !part.is_empty() {
                result.push(
                    part.chars()
                        .next()
                        .unwrap()
                        .to_uppercase()
                        .to_string()
                        .chars()
                        .next()
                        .unwrap(),
                );
                result.push_str(&part[1..]);
            }
        }
        result
    }

    /// Convert to PascalCase
    pub fn to_pascal_case(s: &str) -> String {
        let parts: Vec<&str> = s.split('_').collect();
        parts
            .iter()
            .map(|part| {
                if part.is_empty() {
                    String::new()
                } else {
                    let mut chars = part.chars();
                    let first = chars.next().unwrap().to_uppercase().to_string();
                    first + chars.as_str()
                }
            })
            .collect()
    }

    /// Convert to SCREAMING_SNAKE_CASE
    pub fn to_screaming_snake_case(s: &str) -> String {
        s.to_uppercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optional_field() {
        let field: OptionalField<String> = OptionalField::some("test".to_string());
        assert!(field.is_some());
        assert_eq!(field.as_ref(), Some(&"test".to_string()));
    }

    #[test]
    fn test_rename_snake_case() {
        assert_eq!(rename::to_snake_case("firstName"), "first_name");
        assert_eq!(rename::to_snake_case("HTTPServer"), "h_t_t_p_server");
    }

    #[test]
    fn test_rename_camel_case() {
        assert_eq!(rename::to_camel_case("first_name"), "firstName");
        assert_eq!(rename::to_camel_case("user_id"), "userId");
    }

    #[test]
    fn test_rename_pascal_case() {
        assert_eq!(rename::to_pascal_case("first_name"), "FirstName");
        assert_eq!(rename::to_pascal_case("user_id"), "UserId");
    }

    #[test]
    fn test_rename_screaming_snake_case() {
        assert_eq!(rename::to_screaming_snake_case("firstName"), "FIRSTNAME");
    }
}
