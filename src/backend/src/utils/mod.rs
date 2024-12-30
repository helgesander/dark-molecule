use actix_web::http::StatusCode;
use actix_web::{error, http, HttpResponse};
use derive_more::{Display, Error};
use std::fmt::Display;

#[derive(Debug, Display, Error)]
pub(crate) enum AppError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,
    #[display(fmt = "Bad Request")]
    BadRequest,
    #[display(fmt = "Timeout")]
    Timeout
}

impl error::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest => StatusCode::BAD_REQUEST,
            AppError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(http::header::ContentType::html())
            .body(self.to_string())
    }
}

pub enum Role {
    Admin,
    User,
    Developer,
    CompanyOwner
}

impl Role {
    pub fn to_string(&self) -> String {
        match *self {
            Role::Admin => "admin".to_string(),
            Role::User => "user".to_string(),
            Role::Developer => "developer".to_string(),
            Role::CompanyOwner => "company_owner".to_string()
        }
    }
}


