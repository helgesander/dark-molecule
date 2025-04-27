use crate::utils::errors::AppError;
use actix_web::{get, post, web, HttpResponse};

#[post("/")]
pub async fn create_scan_handler() -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Created().finish())
}

#[get("/{id}/results")]
pub async fn get_scan_handler(id: web::Path<i32>) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body("TODO"))
}
