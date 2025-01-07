use actix_web::{get, post, web, HttpResponse, Responder};
use uuid::Uuid;
use crate::utils::AppError;
use actix_web::web::Data;

#[get("/{id}")]
pub async fn get_user_handler(path: web::Path<String>) -> impl Responder {
    let _uuid = path.into_inner();
    HttpResponse::Ok().body("Getting user, just wait...")
    // TODO: implement get user
}

#[post("/")]
pub async fn create_user_handler() -> actix_web::Result<HttpResponse, AppError> {
    todo!();
    // TODO: implement user creation in postgres
}

#[get("/")]
pub async fn get_users_handler() -> impl Responder {
    HttpResponse::Ok().body("Getting all user, just wait...")
    // TODO: implement getting all users
}

