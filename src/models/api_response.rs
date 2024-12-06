use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status_code: u16,
    pub message: String,
    pub data: T,
}