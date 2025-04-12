use actix_web::{get, post, web, HttpResponse};
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use log::error;
use uuid::Uuid;
use crate::dtos::db::TeamForm;
use crate::models::team::Team;
use crate::utils::errors::AppError;

#[post("/")]
pub async fn create_team_handler(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    data: web::Json<TeamForm>,
) -> Result<HttpResponse, AppError> {
    let team_data = data.into_inner();
    let team = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?;
        Team::create_team(&mut conn, &team_data)
            .map_err(|e| {
                error!("Failed to create team: {}", e);
                AppError::DatabaseError
            })
    }).await??;
    Ok(HttpResponse::Ok().json(team))
}

#[get("/")]
pub async fn get_teams_handler(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, AppError> {
    let teams = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::DatabaseError
        })?;
        Team::get_teams(&mut conn)
            .map_err(|e| {
                error!("Failed to get teams: {}", e);
                AppError::DatabaseError
            })

    }).await??;
    Ok(HttpResponse::Ok().json(teams))
}


#[get("/{team_id}")]
pub async fn get_team_handler(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    team_id: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let id = team_id.into_inner()
        .parse::<Uuid>()
        .map_err(|_| AppError::BadRequest)?;
    let team = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?;
        Team::get_team(&mut conn, id)
            .map_err(|e| {
                error!("Failed to get team: {}", e);
                AppError::DatabaseError
            })
    }).await??;
    Ok(HttpResponse::Ok().json(team))
}