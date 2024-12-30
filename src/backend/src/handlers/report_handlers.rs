use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;
use crate::models;

#[post("/")]
pub async fn create_report_handler(req: HttpRequest) -> impl Responder {
    HttpResponse::Created()
    // TODO: implement report creation
}

#[get("/{id}")]
pub async fn get_report_handler(path: web::Path<Uuid>) -> impl Responder {
    HttpResponse::Ok()
    // TODO: implement get report
}