use crate::utils::errors::AppError;
use actix_web::{get, put, web, HttpResponse};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

#[get("/settings")]
pub async fn get_admin_settings_handler(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().finish())
}

#[put("/settings")]
pub async fn change_admin_settings_handler(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().finish())
}
