use argon2::{Argon2, PasswordHash, PasswordVerifier};

pub fn verify_password(hash: &str, password: &str) -> Result<bool, Box<dyn std::error::Error>> {
   let parsed_hash = PasswordHash::new(hash).map_err(|e| {
      eprintln!("Failed to hashing password {:?}", e);
      "failed hashing password"
   })?;
   let argon2 = Argon2::default();
   match argon2.verify_password(password.as_bytes(), &parsed_hash) {
      Ok(_) => Ok(true),
      Err(_) => Ok(false),
   }
}