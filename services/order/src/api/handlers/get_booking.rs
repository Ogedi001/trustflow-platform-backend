use common::http::ApiResponse;

pub async fn get_booking() -> ApiResponse<&'static str> {
    ApiResponse::success("fetched succesfully", "booking data")
}
