use crate::core::kinds::DatabaseError;
use crate::http::ApiError;

impl From<DatabaseError> for ApiError {
    fn from(error: DatabaseError) -> Self {
        ApiError::internal("Database error occurred")
    }
}
