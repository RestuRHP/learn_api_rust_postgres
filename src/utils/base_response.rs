use crate::models::api_response::ApiResponse;

pub fn base_response<T>(status_code: u16, message: &str, data: Option<T>) -> ApiResponse<T> {
   ApiResponse {
      status_code,
      message: message.to_string(),
      data,
   }
}