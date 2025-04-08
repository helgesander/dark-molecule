use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use uuid::Uuid;
use diesel::{PgConnection, QueryResult};
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use log::{error, debug};
use crate::dtos::db::UserForm;
use crate::models::user::User;
use crate::utils::{AppError, hash_password, Pagination};
use crate::dtos::handlers::UserData;
use diesel::r2d2::Error::ConnectionError;
use validator::Validate;

#[get("/{id}")]
pub async fn get_user_handler(
    path: web::Path<String>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, AppError> {
    let uuid = path.into_inner()
            .parse::<Uuid>()
            .map_err(|e| {
                error!("Invalid UUID format: {}", e);
                AppError::BadRequest
            })?;

    let user = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?;

        User::get_user_by_id(&mut conn, uuid)
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
    match user {
        Ok(Some(user)) => {
            let user_data = UserData::new(&user);
            Ok(HttpResponse::Ok().json(user_data))
        }
            Ok(None) => Ok(HttpResponse::NotFound().json("User not found")),
        Err(err) => {
            error!("Database query error: {}", err);
            Err(AppError::InternalServerError)
        }
    }
}
#[post("/")]
pub async fn create_user_handler(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    data: web::Json<UserForm>,
) -> Result<HttpResponse, AppError> {
    let mut user_data = data.into_inner();
    match user_data.validate() {
        Ok(_) => (),
        Err(_) => Err(AppError::BadRequest)?,
    }
    let created_user = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?; // TODO: fix this shit...
        user_data.password = hash_password(&*user_data.password)?;
        let user = User::create_user(&mut conn, &user_data)?;
        Ok::<_, AppError>(user)
    })
        .await
        .map_err(|e| {
            error!("Async block error: {}", e);
            AppError::InternalServerError
        })??;

    Ok(HttpResponse::Created().json(created_user))
}

#[put("/{id}")]
pub async fn change_user_handler(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    data: web::Json<UserForm>,
) -> Result<HttpResponse, AppError> {
    todo!()
}

// TODO: fix that only admin can delete user
#[delete("/{id}")]
pub async fn delete_user_handler(
    path: web::Path<String>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, AppError> {
    let uuid = path.into_inner()
        .parse::<Uuid>()
        .map_err(|e| {
            error!("Invalid UUID format: {}", e);
            AppError::BadRequest
        })?;
    let deleted_user = web::block( move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?; // TODO: fix this shit...
        debug!("Try to delete user with id {}", uuid);
        User::delete_user(&mut conn, uuid)
        .map_err(|e| {
            error!("Database query error: {}", e);
            AppError::DatabaseError
        })
    })
        .await
        .map_err(|e| {
            error!("Async block error: {}", e);
            AppError::InternalServerError
        })??;
    // TODO: fix response from server
    Ok(HttpResponse::Ok().json(deleted_user))
}

#[get("/")]
pub async fn get_users_handler(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    size: web::Query<Pagination>
) -> actix_web::Result<HttpResponse, AppError> {
    let users = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?;
        // TODO: try understand why way without map_err here dosen't work
        User::get_users(&mut conn, size.size)
            .map_err(|e| {
                error!("Failed to get users: {}", e);
                AppError::DatabaseError
            })
    })
        .await
        .map_err(|e| {
            error!("Async block error: {}", e);
            AppError::InternalServerError
        })??;

    Ok(HttpResponse::Ok().json(users))
}
