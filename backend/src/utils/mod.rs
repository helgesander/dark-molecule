use actix_web::{
    error,
    http::{StatusCode, header},
    HttpResponse};
use derive_more::{Display, Error};
use diesel::r2d2::Error as R2D2Error;
use log::error;
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

#[derive(Debug, Display, Error)]
pub enum AppError {
    #[display("Internal Server Error")]
    InternalServerError,
    #[display("Bad Request")]
    BadRequest,
    #[display("Timeout")]
    Timeout,
    #[display("Internal Server Error")]
    DatabaseError
}

#[derive(Serialize)]
struct AppErrorJson {
    status: usize,
    error: &'static str
}

impl From<R2D2Error> for AppError {
    fn from(error: R2D2Error) -> Self {
        error!("Database pool error: {}", error);
        AppError::InternalServerError
    }
}

impl From<DieselError> for AppError {
    fn from(e: DieselError) -> Self {
        error!("Database error: {}", e);
        AppError::DatabaseError
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(e: argon2::password_hash::Error) -> Self {
        error!{"Error during hash: {}", e};
        AppError::InternalServerError
    }
}

impl error::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest => StatusCode::BAD_REQUEST,
            AppError::Timeout => StatusCode::GATEWAY_TIMEOUT,
            AppError::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    fn error_response(&self) -> HttpResponse {
        match *self {
            AppError::InternalServerError => HttpResponse::Ok().json(AppErrorJson{ status: 505, error: "Internal Server Error" }),
            AppError::BadRequest => HttpResponse::BadRequest().json(AppErrorJson{ status: 400, error: "Bad Request" }),
            AppError::Timeout => HttpResponse::TooManyRequests().append_header((header::LOCATION, "/")).finish(),
            AppError::DatabaseError => HttpResponse::InternalServerError().json(AppErrorJson{ status: 505, error: "Internal Server Error" }),
        }
    }
}

#[derive(Deserialize)]
pub struct Pagination {
    pub size: usize
}

// TODO: change location of this functions

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    Ok(argon2.hash_password(password.as_bytes(), &salt)?.to_string())
}

// TODO: maybe change return type to Result<bool, argon2::Error>
pub fn verify_password(hash: &str, password: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).unwrap();
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}
