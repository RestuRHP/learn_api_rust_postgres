use crate::{
   db::DbPool,
   models::api_response::ApiResponse,
   models::user::user:: User,
   models::user::user_response::UserResponse,
   schema::tb_user,
   utils::base_response::base_response,
};
use diesel::prelude::*;
use rocket::{get, serde::json::Json, State};

#[get("/users")]
pub fn get_all_users(
   pool: &State<DbPool>,
) -> Result<Json<ApiResponse<Vec<UserResponse>>>, Json<ApiResponse<String>>> {
   let mut conn = pool.get().map_err(|_| {
      Json(base_response(
         500,
         "Failed to get DB connection",
         None::<String>,
      ))
   })?;

   let users = tb_user::table
      .load::<User>(&mut conn)
      .map_err(|e| Json(base_response(500, &format!("Error: {}", e), None)))?;

   let user_responses: Vec<UserResponse> = users.iter().map(UserResponse::from_user).collect();

   Ok(Json(base_response(200, "Success", Some(user_responses))))
}