//! Verification configuration
//!
//! Provides configuration types for identity verification (KYC) in the identity service.

use serde::{Deserialize, Serialize};
use time::Duration;

/// Verification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationConfig {
    /// Maximum document upload size in bytes (default: 5MB)
    pub document_upload_max_size: usize,
    /// Allowed document MIME types
    pub allowed_document_types: Vec<String>,
    /// Verification levels that are auto-approved
    pub auto_approve_levels: Vec<u8>,
    /// Manual review threshold (verification level)
    pub manual_review_threshold: u8,
    /// Whether to enable facial recognition verification
    pub facial_recognition_enabled: bool,
    /// Whether to enable liveness detection
    pub liveness_detection_enabled: bool,
    /// Maximum retry attempts for document upload
    pub max_upload_retries: u32,
    /// Document expiration days (for time-limited verification)
    pub document_expiration_days: u32,
    /// Whether to require address verification
    pub address_verification_required: bool,
    /// Whether to enable background checks
    pub background_checks_enabled: bool,
}

impl Default for VerificationConfig {
    fn default() -> Self {
        Self::from_env()
    }
}

impl VerificationConfig {
    /// Create configuration from environment variables
    pub fn from_env() -> Self {
        Self {
            document_upload_max_size: std::env::var("VERIFICATION_DOCUMENT_MAX_SIZE")
                .unwrap_or_else(|_| "5242880".to_string()) // 5MB
                .parse()
                .unwrap_or(5242880),
            allowed_document_types: std::env::var("VERIFICATION_ALLOWED_DOCUMENT_TYPES")
                .unwrap_or_else(|_| "image/jpeg,image/png,image/gif,application/pdf".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            auto_approve_levels: std::env::var("VERIFICATION_AUTO_APPROVE_LEVELS")
                .unwrap_or_else(|_| "0,1".to_string())
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect(),
            manual_review_threshold: std::env::var("VERIFICATION_MANUAL_REVIEW_THRESHOLD")
                .unwrap_or_else(|_| "2".to_string())
                .parse()
                .unwrap_or(2),
            facial_recognition_enabled: std::env::var("VERIFICATION_FACIAL_RECOGNITION_ENABLED")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            liveness_detection_enabled: std::env::var("VERIFICATION_LIVENESS_DETECTION_ENABLED")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            max_upload_retries: std::env::var("VERIFICATION_MAX_UPLOAD_RETRIES")
                .unwrap_or_else(|_| "3".to_string())
                .parse()
                .unwrap_or(3),
            document_expiration_days: std::env::var("VERIFICATION_DOCUMENT_EXPIRATION_DAYS")
                .unwrap_or_else(|_| "365".to_string())
                .parse()
                .unwrap_or(365),
            address_verification_required: std::env::var("VERIFICATION_ADDRESS_REQUIRED")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
            background_checks_enabled: std::env::var("VERIFICATION_BACKGROUND_CHECKS_ENABLED")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
        }
    }

    /// Create configuration from a loader
    pub fn from_loader(loader: &crate::sources::ConfigLoader) -> crate::core::ConfigResult<Self> {
        let allowed_types: Vec<String> = loader
            .get_or(
                "VERIFICATION_ALLOWED_DOCUMENT_TYPES",
                "image/jpeg,image/png,image/gif,application/pdf".to_string(),
            )?
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let auto_approve: Vec<u8> = loader
            .get_or("VERIFICATION_AUTO_APPROVE_LEVELS", "0,1".to_string())?
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        Ok(Self {
            document_upload_max_size: loader
                .get_or("VERIFICATION_DOCUMENT_MAX_SIZE", 5242880usize)?,
            allowed_document_types: allowed_types,
            auto_approve_levels: auto_approve,
            manual_review_threshold: loader.get_or("VERIFICATION_MANUAL_REVIEW_THRESHOLD", 2u8)?,
            facial_recognition_enabled: loader
                .get_or("VERIFICATION_FACIAL_RECOGNITION_ENABLED", false)?,
            liveness_detection_enabled: loader
                .get_or("VERIFICATION_LIVENESS_DETECTION_ENABLED", false)?,
            max_upload_retries: loader.get_or("VERIFICATION_MAX_UPLOAD_RETRIES", 3u32)?,
            document_expiration_days: loader
                .get_or("VERIFICATION_DOCUMENT_EXPIRATION_DAYS", 365u32)?,
            address_verification_required: loader.get_or("VERIFICATION_ADDRESS_REQUIRED", false)?,
            background_checks_enabled: loader
                .get_or("VERIFICATION_BACKGROUND_CHECKS_ENABLED", false)?,
        })
    }

    /// Validate the configuration
    pub fn validate(&self) -> crate::core::ConfigResult<()> {
        if self.document_upload_max_size == 0 {
            return Err(crate::core::ConfigError::validation(
                "Document upload max size must be greater than 0",
            ));
        }
        if self.allowed_document_types.is_empty() {
            return Err(crate::core::ConfigError::validation(
                "At least one document type must be allowed",
            ));
        }
        if self.manual_review_threshold > 3 {
            return Err(crate::core::ConfigError::validation(
                "Manual review threshold must be between 0 and 3",
            ));
        }
        Ok(())
    }

    /// Check if a document type is allowed
    pub fn is_document_type_allowed(&self, mime_type: &str) -> bool {
        self.allowed_document_types.iter().any(|t| t == mime_type)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_verification_config_defaults() {
//         let config = VerificationConfig::default();
//         assert!(!config.allowed_document_types.is_empty());
//         assert!(config.document_upload_max_size > 0);
//     }

//     #[test]
//     fn test_verification_config_validation() {
//         let mut config = VerificationConfig::default();
//         assert!(config.validate().is_ok());

//         config.document_upload_max_size = 0;
//         assert!(config.validate().is_err());
//     }

//     #[test]
//     fn test_is_document_type_allowed() {
//         let config = VerificationConfig::default();
//         assert!(config.is_document_type_allowed("image/jpeg"));
//         assert!(!config.is_document_type_allowed("application/xml"));
//     }
// }
