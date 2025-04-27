pub mod config;
pub mod errors;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct FilterObjects {
    pub size: usize,
    pub name: String,
}

#[derive(Serialize)]
pub struct ResponseJson {
    // pub status: usize,
    pub message: String,
}

// TODO: change location of this functions

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    Ok(argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}

// TODO: maybe change return type to Result<bool, argon2::Error>
pub fn verify_password(hash: &str, password: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).unwrap();
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
