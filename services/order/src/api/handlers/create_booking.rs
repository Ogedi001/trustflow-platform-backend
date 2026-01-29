use common::http::ApiResponse;

pub async fn create_booking() -> ApiResponse<&'static str> {
    ApiResponse::success("booking created successfully", "Booked")
        .with_status(axum::http::StatusCode::CREATED)
}
