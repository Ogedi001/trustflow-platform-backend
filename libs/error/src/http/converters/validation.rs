//! Converter for validation errors to HTTP API errors

use crate::core::kinds::ValidationError;
use crate::http::{ApiError, FieldError};

impl From<ValidationError> for ApiError {
    fn from(error: ValidationError) -> Self {
        let message = error.message.clone();
        let field_errors = if let Some(field) = error.field {
            vec![FieldError {
                field,
                message: message.clone(),
            }]
        } else {
            vec![]
        };

        if field_errors.is_empty() {
            ApiError::validation_error(message)
        } else {
            ApiError::validation_error_with_fields(message, field_errors)
        }
    }
}
