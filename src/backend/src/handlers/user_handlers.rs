use actix_web::{get, post, web, HttpResponse, Responder};
use uuid::Uuid;
use crate::dto::{CreateUserRequestDto, CreateUserResponseDto};
use crate::utils::AppError;

#[get("/{id}")]
pub async fn get_user_handler(path: web::Path<Uuid>) -> impl Responder {
    let _uuid = path.into_inner();
    HttpResponse::Ok().body("Getting user, just wait...")
    // TODO: implement get user
}

#[post("/")]
pub async fn create_user_handler(user: web::Json<CreateUserRequestDto>) -> actix_web::Result<HttpResponse, AppError> {
    let response = CreateUserResponseDto::new(user.get_username(), user.get_company());
    Ok(HttpResponse::Created().json(response))
    // TODO: implement user creation in mongodb
}

#[get("/")]
pub async fn get_users_handler() -> impl Responder {
    HttpResponse::Ok().body("Getting all user, just wait...")
    // TODO: implement getting all users
}

