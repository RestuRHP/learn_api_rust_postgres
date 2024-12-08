use crate::{
    auth::token::Claims,
    db::DbPool,
    models::api_response::ApiResponse,
    models::user::login_request::LoginRequest,
    models::user::user::User,
    models::user::user_tokens::UserToken,
    schema::{tb_user, user_tokens},
    utils::base_response::base_response,
    utils::verify_password::verify_password
};
use chrono::{Duration, Utc};
use diesel::prelude::*;
use jsonwebtoken::{encode, Header};
use rocket::{ post, serde::json::Json, State};
use std::env;
use uuid::Uuid;

#[post("/login", data = "<login_request>")]
pub fn login(
    pool: &State<DbPool>,
    login_request: Json<LoginRequest>,
) -> Result<Json<ApiResponse<String>>, Json<ApiResponse<String>>> {
    let login_request = login_request.into_inner();

    let mut conn = pool.get().map_err(|_| {
        Json(base_response(
            500,
            "Failed to get DB connection",
            None::<String>,
        ))
    })?;

    let user: User = tb_user::table
        .filter(tb_user::email.eq(login_request.email))
        .first::<User>(&mut conn)
        .map_err(|_| Json(base_response(404, "User not found", None)))?;

    if !verify_password(&user.password, &login_request.password).map_err(|e| {
        Json(base_response(
            500,
            &format!("Error verifying password: {}", e),
            None,
        ))
    })? {
        return Err(Json(base_response(401, "Invalid credentials", None)));
    }

    let my_claims = Claims {
        sub: user.id.to_string(),
        exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
    };

    let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let token = encode(
        &Header::default(),
        &my_claims,
        &jsonwebtoken::EncodingKey::from_secret(secret_key.as_bytes()),
    )
    .map_err(|_| Json(base_response(500, "Error creating token", None)))?;

    let user_token = UserToken {
        id: Uuid::new_v4(),
        user_id: user.id,
        token: token.clone(),
        created_at: Some(Utc::now()),
        expired_at: Some(Utc::now() + Duration::hours(1)),
    };

    diesel::insert_into(user_tokens::table)
        .values(&user_token)
        .execute(&mut conn)
        .map_err(|_| Json(base_response(500, "Error saving token", None)))?;

    Ok(Json(base_response(200, "Login successful", Some(token))))
}