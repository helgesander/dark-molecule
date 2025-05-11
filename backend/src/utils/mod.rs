pub mod config;
pub mod errors;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};
use log::debug;
use crate::utils::errors::AppError;

#[derive(Deserialize)]
pub struct FilterObjects {
    pub size: Option<usize>,
    pub page: Option<usize>,
    pub name: Option<String>,
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

pub fn verify_password(hash: &str, password: &str) -> Result<bool, AppError> {
    debug!("Verifying password: {}", hash);
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| {
            debug!("Error parsing password hash: {}", e);
            AppError::InternalServerError
        })?;

    match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(argon2::password_hash::Error::Password) => Ok(false),
        Err(e) => {
            debug!("Error verifying password: {}", e);
            Err(AppError::InternalServerError)
        }
    }
}
