use argon2::password_hash::SaltString;
use argon2::{password_hash::PasswordHasher, Algorithm, Argon2, Params, Version};
use dotenv::dotenv;
use std::env;

pub fn hash_password(password: &str) -> Result<String, Box<dyn std::error::Error>> {
    dotenv().ok();
    let env_salt = env::var("SALT").expect("SALT must be set");
    println!("Using Salt: {}", env_salt);

    let salt = SaltString::from_b64(&env_salt).map_err(|e| {
        eprintln!("Failed to parse salt: {:?}", e);
        "Failed to parse salt from environment variable"
    })?;

    let params = Params::new(4096, 3, 1, None).map_err(|e| {
        eprintln!("Failed to create params {:?}", e);
        "failed create params"
    })?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| {
            eprintln!("Failed to hash password: {:?}", e);
            "failed to hash password"
        })?
        .to_string();

    Ok(password_hash)
}
