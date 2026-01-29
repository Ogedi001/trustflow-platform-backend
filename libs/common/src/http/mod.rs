pub mod axum;
pub mod error;
pub mod error_code;
pub mod fallback;
pub mod health;
pub mod meta;
pub mod pagination;
pub mod response;

pub use error::ApiError;
pub use response::{ApiResponse, ApiResult};
pub use error_code::ErrorCode;
