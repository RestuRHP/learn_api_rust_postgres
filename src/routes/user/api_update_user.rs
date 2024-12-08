use crate::{
   auth::middleware::AuthenticatedUser,
   db::DbPool,
   models::api_response::ApiResponse,
   models::user::user::{NewUser, User},
   models::user::user_response::UserResponse,
   schema::tb_user,
   utils::base_response::base_response,
};
use chrono::{FixedOffset, Utc};
use diesel::prelude::*;
use rocket::{ put, serde::json::Json, State};
use uuid::Uuid;

#[put("/users/<id>", data = "<updated_user>")]
pub fn update_user(
   pool: &State<DbPool>,
   id: &str,
   updated_user: Json<NewUser>,
   auth_user: AuthenticatedUser,
) -> Result<Json<ApiResponse<UserResponse>>, Json<ApiResponse<String>>> {
   let id = Uuid::parse_str(id)
      .map_err(|_| Json(base_response(400, "Invalid UUID format", None::<String>)))?;

   if auth_user.0 != id {
      return Err(Json(base_response(
         400,
         "Unauthorized access",
         None::<String>,
      )));
   }

   let mut conn = pool
      .get()
      .map_err(|_| Json(base_response(500, "Failed to get DB connection", None)))?;

   let jakarta_offset = FixedOffset::east_opt(7 * 3600).expect("Invalid timezone offset");
   let now_in_jakarta = Utc::now().with_timezone(&jakarta_offset).naive_local();

   diesel::update(tb_user::table.filter(tb_user::id.eq(id)))
      .set((
         tb_user::name.eq(updated_user.name.clone()),
         tb_user::email.eq(updated_user.email.clone()),
         tb_user::address.eq(updated_user.address.clone()),
         tb_user::update_at.eq(now_in_jakarta),
      ))
      .get_result::<User>(&mut conn)
      .map(|user| {
         let user_response = UserResponse::from_user(&user);
         Json(base_response(
            200,
            "User updated successfully",
            Some(user_response),
         ))
      })
      .or_else(|e| match e {
         diesel::result::Error::NotFound => {
            Err(Json(base_response(404, "User not found!", None)))
         }
         _ => Err(Json(base_response(
            500,
            &format!("Internal server error!: {}", e),
            None,
         ))),
      })
}