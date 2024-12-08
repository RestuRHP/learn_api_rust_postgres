use rocket::request::{self, FromRequest, Request};
use rocket::http::Status;
use crate::auth::token::verify_token;
use uuid::Uuid;

pub struct AuthenticatedUser(pub Uuid);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
   type Error = ();

   async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
      let keys: &String = req.rocket().state::<String>().unwrap();
      let auth_header = req.headers().get_one("Authorization");

      if let Some(auth) = auth_header {
         if let Some(token) = auth.strip_prefix("Bearer ") {
            return match verify_token(token, keys.as_str()) {
               Ok(user_id) => request::Outcome::Success(AuthenticatedUser(user_id)),
               Err(_) => request::Outcome::Error((Status::Unauthorized, ())),
            }
         }
      }

      request::Outcome::Error((Status::Unauthorized, ()))
   }
}
