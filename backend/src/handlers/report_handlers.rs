use actix_web::{get, post, HttpRequest, HttpResponse};
use crate::utils::errors::AppError;

#[post("/")]
pub async fn create_report_handler() -> Result<HttpResponse, AppError> {
    unimplemented!()
}

#[get("/{id}")]
pub async fn get_report_handler(req: HttpRequest) -> Result<HttpResponse, AppError> {
    unimplemented!()
}


#[get("/")]
pub async fn get_reports_handler() -> Result<HttpResponse, AppError> {
    unimplemented!()
}