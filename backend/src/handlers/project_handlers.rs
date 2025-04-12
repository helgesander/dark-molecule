use actix_web::{delete, get, post, put, web, HttpResponse};
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use log::error;
use uuid::Uuid;
use crate::utils::errors::AppError;
use crate::dtos::handlers::{IssueForm, ProjectForm, HostForm};
use crate::models::host::Host;
use crate::models::issue::Issue;
use crate::models::project::Project;

#[get("/")]
pub async fn get_projects_handler(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>
) -> Result<HttpResponse, AppError> {
    let projects = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?;
        Project::get_projects(&mut conn)
            .map_err(|e| {
                error!("Failed to get all projects overview: {}", e);
                AppError::DatabaseError
            })
    })
    .await??;
    Ok(HttpResponse::Ok().json(projects))
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
    let project_id_str = path.into_inner();
    let project_id = Uuid::parse_str(&project_id_str)
        .map_err(|_| AppError::BadRequest)?;
    let project = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?;

        Project::get_project_by_id(&mut conn, project_id)
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
        Ok(Some(project)) => Ok(HttpResponse::Ok().json(project)),
        Ok(None) => Err(AppError::NotFound),
        Err(err) => {
            error!("Database query error: {}", err);
            Err(err)
        }
    }
}

/// Handler to get minimized response for project (name and something else if need for frontend)
#[get("/{id}/overview")]
pub async fn get_project_overview_handler(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<String>
) -> Result<HttpResponse, AppError> {
    unimplemented!();
}

#[get("/{id}/issues")] //TODO: мне название эндпоинта не нравится, но я пока оставлю так
pub async fn get_issues_handler(
    id: web::Path<String>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, AppError> {
    let id = id.into_inner()
        .parse::<Uuid>()
        .map_err(|_| AppError::BadRequest)?;
    let issues = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?;

        Issue::get_issues_by_project_id(&mut conn, id)
            .map_err(|e| {
                error!("Can't get issues by project id: {}", e);
                AppError::DatabaseError
            })
    }).await??;
    Ok(HttpResponse::Ok().json(issues))
}

#[get("/{id}/hosts")]
pub async fn get_hosts_handler(
    id: web::Path<String>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, AppError> {

    let id = id.into_inner()
        .parse::<Uuid>()
        .map_err(|_| AppError::BadRequest)?;

    let hosts = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?;
        Host::get_hosts_by_project_id(&mut conn, id)
            .map_err(|e| {
                error!("Failed to get hosts by project id: {}", e);
                AppError::DatabaseError
            })
    }).await??;
    Ok(HttpResponse::Ok().json(hosts))
}


#[delete("/{project_id}/issue/{issue_id}")]
pub async fn delete_issue_handler(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<(String, String)>
) -> Result<HttpResponse, AppError> {
    let (_, issue_uuid_str) = path.into_inner();
    let issue_id = Uuid::parse_str(&issue_uuid_str)
        .map_err(|_| AppError::BadRequest)?;
    let count = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?;
        Issue::delete_issue(&mut conn, issue_id)
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

#[put("/{project_id}/issue/{issue_id}")]
pub async fn update_issue_handler(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<(String, String)>,
    form: web::Json<IssueForm>
) -> Result<HttpResponse, AppError> {
    let update_data = form.into_inner();
    let (project_uuid_str, issue_uuid_str) = path.into_inner();
    let project_id = Uuid::parse_str(&project_uuid_str)
        .map_err(|_| AppError::BadRequest)?;
    let issue_id = Uuid::parse_str(&issue_uuid_str)
        .map_err(|_| AppError::BadRequest)?;
    let count = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?;
        Issue::update_issue(&mut conn, &update_data, project_id, issue_id)
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

#[post("/{project_id}/issue")]
pub async fn create_issue_handler(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<String>,
    data: web::Json<IssueForm>
) -> Result<HttpResponse, AppError> {
    let issue_data = data.into_inner();
    let project_id_str = path.into_inner();
    let project_id = Uuid::parse_str(&project_id_str)
        .map_err(|_| AppError::BadRequest)?;
    let issue = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?;

        Issue::create_issue(&mut conn, &issue_data, project_id)
            .map_err(|e| {
                error!("Failed to create issue: {}", e);
                AppError::DatabaseError
            })
    }).await??;
    Ok(HttpResponse::Ok().json(issue))
}

#[get("/{project_id}/issue/{issue_id}")]
pub async fn get_issue_handler(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<(String, String)>, // TODO: maybe change to Uuid type inside
) -> Result<HttpResponse, AppError> {
    let (_, issue_id_str) = path.into_inner();
    let issue_id = Uuid::parse_str(&issue_id_str)
        .map_err(|_| AppError::BadRequest)?;
    let issue = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?;
        Issue::get_issue(&mut conn, issue_id)
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


#[get("/{project_id}/host/{host_id}")]
pub async fn get_host_handler(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<(Uuid, i32)>
) -> Result<HttpResponse, AppError> {
    let (_, host_id) = path.into_inner();
    let host = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?;
        Host::get_host(&mut conn, host_id)
            .map_err(|e| {
                error!("Failed to get hosts by project id: {}", e);
                AppError::DatabaseError
            })
    }).await??;

    Ok(HttpResponse::Ok().json(host))
}

#[post("/{project_id}/host")]
pub async fn create_host_handler(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<String>,
    data: web::Json<HostForm>
) -> Result<HttpResponse, AppError> {
    let project_id_str = path.into_inner();
    let project_id = Uuid::parse_str(&project_id_str)
        .map_err(|_| AppError::BadRequest)?;
    let host = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?;
        Host::create_host(&mut conn, &data.into_inner(), project_id)
            .map_err(|e| {
                error!("Failed to create host: {}", e);
                AppError::DatabaseError
            })
    }).await??;
    Ok(HttpResponse::Ok().json(host))
}


#[put("/{project_id}/host/{host_id}")]
pub async fn update_host_handler(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    path: web::Path<(String, i32)>,
    data: web::Json<HostForm>
) -> Result<HttpResponse, AppError> {
    let (project_id_str, host_id) = path.into_inner();
    let project_id = Uuid::parse_str(&project_id_str)
        .map_err(|_| AppError::BadRequest)?;
    let count = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?;
        Host::update_host(&mut conn, &data.into_inner(), project_id, host_id)
            .map_err(|e| {
                error!("Failed to update host by project id: {}", e);
                AppError::DatabaseError
            })
    }).await??;
    match count {
        1 => Ok(HttpResponse::Ok().finish()),
        0 => Err(AppError::NotFound),
        _ => Err(AppError::InternalServerError),
    }
}