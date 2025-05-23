use std::io::Read;

use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use actix_web::{get, post, web, HttpResponse};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use log::{debug, error};

use crate::dtos::handlers::{ReportTemplateForm, UploadReportTemplateForm};
use crate::models::report_template::*;
use crate::utils::errors::AppError;

#[get("/all")]
pub async fn get_templates_preview_handler(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, AppError> {
    let templates = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?;
        ReportTemplate::get_all_templates(&mut conn).map_err(|e| {
            error!("Failed to get templates: {}", e);
            AppError::DatabaseError
        })
    })
    .await??;
    Ok(HttpResponse::Ok().json(templates))
}

#[get("/{id}")]
pub async fn get_template_handler(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let id = path.into_inner();
    let template = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?;
        ReportTemplate::get_template_by_id(&mut conn, id).map_err(|e| {
            error!("Failed to get template: {}", e);
            AppError::DatabaseError
        })
    })
    .await??;
    Ok(HttpResponse::Ok().json(template))
}

#[post("/")]
pub async fn create_template_handler(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    MultipartForm(form): MultipartForm<UploadReportTemplateForm>,
) -> Result<HttpResponse, AppError> {
    let (file_data, filename) = if let Some(some_file) = form.file {
        let mut data: Vec<u8> = Vec::new();
        let mut file = some_file.file.as_file();
        let filename = some_file.file_name.ok_or_else(|| {
            error!("Missing filename");
            AppError::BadRequest
        })?;
        file.read_to_end(&mut data).map_err(|e| {
            error!("Error for open file: {}", e);
            AppError::BadRequest
        })?;
        (data, filename)
    } else {
        return Err(AppError::BadRequest);
    };

    let new_template = ReportTemplateForm {
        file: file_data,
        filename,
        name: form.name.clone(),
    };

    debug!("Get new template: {}", new_template.name);

    let template = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?;
        ReportTemplate::create_template(&mut conn, &new_template).map_err(|e| {
            error!("Failed to create template: {}", e);
            AppError::DatabaseError
        })
    })
    .await??;

    Ok(HttpResponse::Created().json(template))
}
