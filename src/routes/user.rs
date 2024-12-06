use crate::models::api_response::ApiResponse;
use crate::{
   db::DbPool,
   models::user::{NewUser, User},
   schema::tb_user,
};
use diesel::prelude::*;
use rocket::{delete, get, post, put, serde::json::Json, State};
use uuid::Uuid;

fn create_response<T>(status_code: u16, message: &str, data: Option<T>) -> ApiResponse<T> {
   ApiResponse {
      status_code,
      message: message.to_string(),
      data,
   }
}

#[get("/users")]
pub fn get_all_users(
   pool: &State<DbPool>,
) -> Result<Json<ApiResponse<Vec<User>>>, Json<ApiResponse<String>>> {
   let mut conn = pool.get().map_err(|_| {
      Json(create_response(
         500,
         "Failed to get DB connection",
         None::<String>,
      ))
   })?;

   let users = tb_user::table
      .load::<User>(&mut conn)
      .map_err(|e| Json(create_response(500, &format!("Error: {}", e), None)))?;

   Ok(Json(create_response(200, "Success", Some(users))))
}

#[get("/users/<id>")]
pub fn get_user_by_id(
   pool: &State<DbPool>,
   id: &str,
) -> Result<Json<ApiResponse<User>>, Json<ApiResponse<String>>> {
   let id = Uuid::parse_str(id).map_err(|_| {
      Json(create_response(400, "Invalid UUID format", None))
   })?;

   let mut conn = pool.get().map_err(|_| {
      Json(create_response(
         500,
         "Failed to get DB connection",
         None::<String>,
      ))
   })?;

   tb_user::table
      .filter(tb_user::id.eq(id))
      .first::<User>(&mut conn)
      .map(|user| Json(create_response(200, "User found", Some(user))))
      .or_else(|e| match e {
         diesel::result::Error::NotFound => Err(Json(create_response(
            404,
            "User not found",
            None,
         ))),
         _ => Err(Json(create_response(
            500,
            &format!("Internal server error: {}", e),
            None,
         ))),
      })
}

#[post("/users", data = "<new_user>")]
pub fn create_user(
   pool: &State<DbPool>,
   new_user: Json<NewUser>,
) -> Result<Json<ApiResponse<User>>, Json<ApiResponse<String>>> {
   let mut conn = pool
      .get()
      .map_err(|_| Json(create_response(500, "Failed to get DB connection", None)))?;

   let user = diesel::insert_into(tb_user::table)
      .values(&*new_user)
      .get_result::<User>(&mut conn)
      .map_err(|e| {
         Json(create_response(
            500,
            &format!("Error inserting user: {:?}", e),
            None,
         ))
      })?;

   Ok(Json(create_response(201, "User created successfully", Some(user))))
}

#[put("/users/<id>", data = "<updated_user>")]
pub fn update_user(
   pool: &State<DbPool>,
   id: &str,
   updated_user: Json<NewUser>,
) -> Result<Json<ApiResponse<User>>, Json<ApiResponse<String>>> {
   let id = Uuid::parse_str(id).map_err(|_| {
      Json(create_response(400, "Invalid UUID format", None::<String>))
   })?;

   let mut conn = pool
      .get()
      .map_err(|_| Json(create_response(500, "Failed to get DB connection", None)))?;

   diesel::update(tb_user::table.filter(tb_user::id.eq(id)))
      .set(&*updated_user)
      .get_result::<User>(&mut conn)
      .map(|user| Json(create_response(200, "User updated successfully", Some(user))))
      .or_else(|e| match e {
         diesel::result::Error::NotFound => Err(Json(create_response(
            404,
            "User not found!",
            None,
         ))),
         _ => Err(Json(create_response(
            500,
            &format!("Internal server error!: {}", e),
            None,
         ))),
      })
}

#[delete("/users/<id>")]
pub fn delete_user(
   pool: &State<DbPool>,
   id: &str,
) -> Result<Json<ApiResponse<String>>, Json<ApiResponse<String>>> {
   let id = Uuid::parse_str(id).map_err(|_| {
      Json(create_response(400, "Invalid UUID format", None::<String>))
   })?;

   let mut conn = pool
      .get()
      .map_err(|_| Json(create_response(500, "Failed to get DB connection", None)))?;

   let rows_deleted = diesel::delete(tb_user::table.filter(tb_user::id.eq(id)))
      .execute(&mut conn)
      .map_err(|e| Json(create_response(500, &format!("Error deleting user: {}", e), None)))?;

   if rows_deleted == 0 {
      return Err(Json(create_response(404, "User not found", None)));
   }

   Ok(Json(create_response(
      200,
      "User deleted successfully",
      Some("User deleted".to_string()),
   )))
}
