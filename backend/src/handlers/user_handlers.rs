use actix_web::{get, post, web, HttpResponse, Responder};
use uuid::Uuid;
use diesel::{PgConnection, QueryResult};
use diesel::r2d2::{ConnectionManager, Pool};
use log::{error, info};
use crate::models::user::User;
use crate::utils::AppError;
use crate::dtos::handlers::UserData;
use diesel::r2d2::Error::ConnectionError;


#[get("/{id}")]
pub async fn get_user_handler(
    path: web::Path<String>,
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, AppError> {
    let uuid = path.into_inner();

    let user = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?;
        info!("Getting user...");
        User::get_user_by_id(&mut conn, uuid.parse().unwrap())
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
pub async fn create_user_handler() -> actix_web::Result<HttpResponse, AppError> {
    todo!();
}

#[get("/")]
pub async fn get_users_handler(size: web::Query<usize>) -> actix_web::Result<HttpResponse, AppError> {
    todo!();
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[actix_web::test]
//     async fn test_create_user_handler() {
//         let
//     }
// }