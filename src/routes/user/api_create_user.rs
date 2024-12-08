use crate::{
   db::DbPool,
   models::api_response::ApiResponse,
   models::user::user::{NewUser, User},
   models::user::user_response::UserResponse,
   schema::tb_user,
   utils::base_response::base_response,
   utils::hash_password::hash_password
};
use diesel::prelude::*;
use rocket::{post,serde::json::Json, State};

#[post("/users", data = "<new_user>")]
pub fn create_user(
   pool: &State<DbPool>,
   new_user: Json<NewUser>,
) -> Result<Json<ApiResponse<UserResponse>>, Json<ApiResponse<String>>> {
   let mut conn = pool
      .get()
      .map_err(|_| Json(base_response(500, "Failed to get DB connection", None)))?;

   let hashed_password = hash_password(&new_user.password).map_err(|e| {
      Json(base_response(
         500,
         &format!("Error hashing password: {}", e),
         None,
      ))
   })?;

   let user = diesel::insert_into(tb_user::table)
      .values(NewUser {
         password: hashed_password,
         ..new_user.into_inner()
      })
      .get_result::<User>(&mut conn)
      .map_err(|e| {
         Json(base_response(
            500,
            &format!("Error inserting user: {:?}", e),
            None,
         ))
      })?;

   let user_response = UserResponse::from_user(&user);

   Ok(Json(base_response(
      201,
      "User created successfully",
      Some(user_response),
   )))
}