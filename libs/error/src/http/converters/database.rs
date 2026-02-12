use crate::core::kinds::DatabaseError;
use crate::http::ApiError;

impl From<DatabaseError> for ApiError {
    fn from(error: DatabaseError) -> Self {
        ApiError::internal(format!("Database error occurred: {}", error.message)).with_details(
            serde_json::json!({
                "message": error.message,
                "code": error.code,
                "source": "database"
            }),
        )
    }
}
