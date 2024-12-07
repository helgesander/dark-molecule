use actix_web::{get, post, web, HttpResponse, Responder};
use uuid::Uuid;

#[get("/{uuid}")]
pub async fn get_user_handler(path: web::Path<Uuid>) -> impl Responder {
    let _uuid = path.into_inner();
    HttpResponse::Ok().body("Getting user, just wait...")
    // TODO: implement get user
}

#[post("/")]
pub async fn create_user_handler() -> impl Responder {
    HttpResponse::Created()
    // TODO: implement user creation
}

#[get("/")]
pub async fn get_users_handler() -> impl Responder {
    HttpResponse::Ok().body("Getting all user, just wait...")
    // TODO: implement getting all users
}

