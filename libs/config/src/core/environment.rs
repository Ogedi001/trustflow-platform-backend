//!
//! Defines the different environments that the application can run in.
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// Application environment types
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Testing,
    Staging,
    Production,
}

impl Environment {
    pub fn is_development(&self) -> bool {
        matches!(self, Self::Development)
    }

    pub fn is_testing(&self) -> bool {
        matches!(self, Self::Testing)
    }

    pub fn is_staging(&self) -> bool {
        matches!(self, Self::Staging)
    }

    pub fn is_production(&self) -> bool {
        matches!(self, Self::Production)
    }

    pub fn allows_debug(&self) -> bool {
        matches!(self, Self::Development | Self::Testing)
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Development => "Development",
            Self::Testing => "Testing",
            Self::Staging => "Staging",
            Self::Production => "Production",
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::Development
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

impl FromStr for Environment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "development" | "dev" => Ok(Self::Development),
            "testing" | "test" => Ok(Self::Testing),
            "staging" | "stage" => Ok(Self::Staging),
            "production" | "prod" => Ok(Self::Production),
            _ => Err(format!("Unknown environment: {}", s)),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_environment_parsing() {
//         assert_eq!("development".parse(), Ok(Environment::Development));
//         assert_eq!("dev".parse(), Ok(Environment::Development));
//         assert_eq!("production".parse(), Ok(Environment::Production));
//         assert_eq!("prod".parse(), Ok(Environment::Production));
//         assert_eq!("testing".parse(), Ok(Environment::Testing));
//         assert_eq!("staging".parse(), Ok(Environment::Staging));
//     }

//     #[test]
//     fn test_environment_display() {
//         assert_eq!(Environment::Development.display_name(), "Development");
//         assert_eq!(Environment::Production.display_name(), "Production");
//     }

//     #[test]
//     fn test_environment_checks() {
//         assert!(Environment::Development.is_development());
//         assert!(!Environment::Production.is_development());
//         assert!(Environment::Production.is_production());
//         assert!(!Environment::Development.is_production());
//     }

//     #[test]
//     fn test_allows_debug() {
//         assert!(Environment::Development.allows_debug());
//         assert!(Environment::Testing.allows_debug());
//         assert!(!Environment::Production.allows_debug());
//         assert!(!Environment::Staging.allows_debug());
//     }
// }
