use crate::{
   auth::middleware::AuthenticatedUser,
   db::DbPool,
   models::api_response::ApiResponse,
   models::user::user::User,
   models::user::user_response::UserResponse,
   schema::tb_user,
   utils::base_response::base_response,
};
use diesel::prelude::*;
use rocket::{ get, serde::json::Json, State};
use uuid::Uuid;

#[get("/users/<id>")]
pub fn get_user_by_id(
   pool: &State<DbPool>,
   id: &str,
   auth_user: AuthenticatedUser,
) -> Result<Json<ApiResponse<UserResponse>>, Json<ApiResponse<String>>> {
   let id =
      Uuid::parse_str(id).map_err(|_| Json(base_response(400, "Invalid UUID format", None)))?;

   if auth_user.0 != id {
      return Err(Json(base_response(
         400,
         "Unauthorized access",
         None::<String>,
      )));
   }

   let mut conn = pool.get().map_err(|_| {
      Json(base_response(
         500,
         "Failed to get DB connection",
         None::<String>,
      ))
   })?;

   let user: Result<User, diesel::result::Error> = tb_user::table
      .filter(tb_user::id.eq(id))
      .first::<User>(&mut conn);

   user.map(|user|{
      let user_response = UserResponse::from_user(&user);
      Json(base_response(200, "User found", Some(user_response)))
   })
      .or_else(|e| match e {
         diesel::result::Error::NotFound => {
            Err(Json(base_response(404, "User not found", None)))
         }
         _ => Err(Json(base_response(
            500,
            &format!("Internal server error: {}", e),
            None,
         ))),
      })
}