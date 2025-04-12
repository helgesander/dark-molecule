use actix_web::{get, post, web, HttpResponse};
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use log::error;
use uuid::Uuid;
use crate::utils::errors::AppError;
use crate::dtos::db::ProjectForm;
use crate::dtos::handlers::ProjectResponse;
use crate::models::project::Project;

#[get("/")]
pub async fn get_projects_handler(

) -> Result<HttpResponse, AppError> {
    unimplemented!()
}

#[post("/")]
pub async fn create_project_handler(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    data: web::Json<ProjectForm>
) -> Result<HttpResponse, AppError> {
    let project_data = data.into_inner();
    let project = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?;
        Project::create_project(&mut conn, &project_data)
            .map_err(|e| {
                error!("Database query error: {}", e);
                AppError::DatabaseError
            })
    }).await??;

    Ok(HttpResponse::Created().json(project))
}

#[get("/{id}")]
pub async fn get_project_handler(
    path: web::Path<String>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, AppError> {
    let uuid = path.into_inner()
        .parse::<Uuid>()
        .map_err(|e| {
            error!("Invalid UUID format: {}", e);
            AppError::BadRequest
        })?;
    let project = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?;

        Project::get_project_by_id(&mut conn, uuid)
            .map_err(|e| {
                error!("Database query error: {}", e);
                AppError::DatabaseError
            })
    })
        .await
        .map_err(|e| {
            error!("Async block error: {}", e);
            AppError::InternalServerError
        })?;
    match project {
        Ok(Some(project)) => {
            let project_data = ProjectResponse::new(&project);
            Ok(HttpResponse::Ok().json(project_data))
        }
        Ok(None) => Ok(HttpResponse::NotFound().json("Project not found")),
        Err(err) => {
            error!("Database query error: {}", err);
            Err(err)
        }
    }
}

#[get("/{id}/issues")]
pub async fn get_issues_handler(
    path: web::Path<Uuid>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, AppError> {
    unimplemented!()
}

#[get("/{id}/hosts")]
pub async fn get_hosts_handler(
    path: web::Path<Uuid>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, AppError> {
    unimplemented!()
}


