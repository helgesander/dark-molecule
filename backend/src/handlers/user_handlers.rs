use crate::dtos::db::UserForm;
use crate::dtos::handlers::UserData;
use crate::models::user::User;
use crate::utils::errors::AppError;
use crate::utils::{hash_password, FilterObjects};
use actix_web::{delete, get, post, put, web, HttpResponse};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use log::{debug, error};
use uuid::Uuid;
use validator::Validate;

#[get("/{id}")]
pub async fn get_user_handler(
    path: web::Path<String>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, AppError> {
    let user_id_str = path.into_inner();
    let user_id = Uuid::parse_str(&user_id_str).map_err(|_| AppError::BadRequest)?;

    let user = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?;

        User::get_user_by_id(&mut conn, user_id).map_err(|e| {
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
        Ok(None) => Err(AppError::NotFound),
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
    .await??;

    Ok(HttpResponse::Created().json(created_user))
}

// TODO: fix put change_user_handler
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
    let user_id_str = path.into_inner();
    let user_id = Uuid::parse_str(&user_id_str).map_err(|_| AppError::BadRequest)?;
    let deleted_user = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?; // TODO: fix this shit...
        debug!("Try to delete user with id {}", user_id);
        User::delete_user(&mut conn, user_id).map_err(|e| {
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
    filter_data: web::Query<FilterObjects>,
) -> actix_web::Result<HttpResponse, AppError> {
    // TODO: change return of all object of user, need create other response struct which will return data without password
    let users = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?;
        // TODO: try understand why way without map_err here dosen't work
        User::get_users(&mut conn, &filter_data).map_err(|e| {
            error!("Failed to get users: {}", e);
            AppError::DatabaseError
        })
    })
    .await??;

    Ok(HttpResponse::Ok().json(users))
}
