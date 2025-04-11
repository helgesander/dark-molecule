use actix_web::{get, post, HttpResponse};
use crate::utils::errors::AppError;

#[get("/")]
pub async fn get_issues_handler(

) -> Result<HttpResponse, AppError> {
    unimplemented!()
}

#[post("/")]
pub async fn create_issue_handler(

) -> Result<HttpResponse, AppError> {
    unimplemented!()
}

#[get("/{id}")]
pub async fn get_issue_handler() -> Result<HttpResponse, AppError> {
    unimplemented!()
}
