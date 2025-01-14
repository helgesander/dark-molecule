
use actix_web::{get, post, HttpResponse, Responder};

#[get("/")]
pub async fn get_projects_handler() -> impl Responder {
    HttpResponse::Ok().body("Getting products, just wait...")
}

#[post("/")]
pub async fn create_project_handler() -> impl Responder {
    HttpResponse::Created()
}

#[get("/{id}")]
pub async fn get_project_handler() -> impl Responder {
    HttpResponse::Ok().body("Getting project")
}
// TODO: implement all project handlers
