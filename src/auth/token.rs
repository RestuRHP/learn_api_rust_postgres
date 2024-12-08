use jsonwebtoken::{decode, Validation, DecodingKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize,Serialize)]
pub struct Claims {
   pub sub: String,
   pub exp: usize,
}

pub fn verify_token(token: &str, secret: &str) -> Result<Uuid, String> {
   match decode::<Claims>(
      token,
      &DecodingKey::from_secret(secret.as_ref()),
      &Validation::default(),
   ) {
      Ok(data) => {
         let user_id = Uuid::parse_str(&data.claims.sub).map_err(|_| "Invalid user ID format")?;
         Ok(user_id)
      }
      Err(_) => Err("Invalid or expired token".into()),
   }
}
