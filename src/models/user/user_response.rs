use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;
use crate::models::user::user::User;

#[derive(Serialize)]
pub struct UserResponse {
   pub id: Uuid,
   pub name: Option<String>,
   pub email: Option<String>,
   pub address: Option<String>,
   pub create_at: Option<DateTime<Utc>>,
   pub update_at: Option<DateTime<Utc>>,
}

impl UserResponse {
   pub fn from_user(user: &User) -> Self {
      UserResponse {
         id: user.id,
         name: user.name.clone(),
         email: user.email.clone(),
         address: user.address.clone(),
         create_at: user.create_at,
         update_at: user.update_at,
      }
   }
}