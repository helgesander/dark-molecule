use actix_web::{delete, get, post, put, web, HttpResponse};
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use log::error;
use uuid::Uuid;
use crate::db::schema::proof_of_concepts::issue_id;
use crate::dtos::db::IssueForm;
use crate::models::issue::Issue;
use crate::utils::errors::AppError;
use crate::utils::ResponseJson;

#[post("/")]
pub async fn create_issue_handler(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    data: web::Json<IssueForm>
) -> Result<HttpResponse, AppError> {
    let issue_data = data.into_inner();
    let issue = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?;

        Issue::create_issue(&mut conn, &issue_data)
            .map_err(|e| {
                error!("Failed to create issue: {}", e);
                AppError::DatabaseError
            })
    }).await??;
    Ok(HttpResponse::Ok().json(issue))
}

#[get("/{id}")]
pub async fn get_issue_handler(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<String>, // TODO: maybe change to Uuid type inside
) -> Result<HttpResponse, AppError> {
    let uuid = path.into_inner()
        .parse::<Uuid>()
        .map_err(|e| {
            error!("Invalid UUID format: {}", e);
            AppError::BadRequest
        })?;
    let issue = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?;
        Issue::get_issue(&mut conn, uuid)
            .map_err(|e| {
                error!("Failed to get issue: {}", e);
                AppError::DatabaseError
            })
    }).await??;

    match issue {
        Some(issue) => Ok(HttpResponse::Ok().json(issue)),
        None => Err(AppError::NotFound)
    }
}


#[put("/{id}")]
pub async fn update_issue_handler(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<String>,
    form: web::Json<IssueForm>
) -> Result<HttpResponse, AppError> {
    let update_data = form.into_inner();
    let uuid = path.into_inner()
        .parse::<Uuid>()
        .map_err(|e| {
            error!("Invalid UUID format: {}", e);
            AppError::BadRequest
        })?;
    let count = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?;
        Issue::update_issue(&mut conn, &update_data, uuid)
            .map_err(|e| {
                error!("Failed to update issue: {}", e);
                AppError::DatabaseError
            })
    }).await??;
    match count {
        1 => Ok(HttpResponse::Ok().finish()),
        0 => Err(AppError::NotFound),
        _ => Err(AppError::InternalServerError),
    }
}

#[delete("/{id}")]
pub async fn delete_issue_handler(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<String>
) -> Result<HttpResponse, AppError> {
    let uuid = path.into_inner()
        .parse::<Uuid>()
        .map_err(|e| {
            error!("Invalid UUID format: {}", e);
            AppError::BadRequest
        })?;
    let count = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?;
        Issue::delete_issue(&mut conn, uuid)
            .map_err(|e| {
                error!("Failed to delete issue: {}", e);
                AppError::DatabaseError
            })
    }).await??;
    match count {
        1 => Ok(HttpResponse::Ok().finish()),
        0 => Err(AppError::NotFound),
        _ => Err(AppError::InternalServerError)
    }
}
