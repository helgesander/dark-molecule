use crate::middleware::auth::{Role, UserSession};
use crate::models::user::User;
use crate::utils::errors::AppError;
use actix_session::Session;
use actix_web::{post, web, HttpResponse};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use log::error;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
struct LoginForm {
    #[validate(email)]
    pub email: String,
    pub password: String,
}
#[post("/")]
pub async fn auth_handler(
    pool: web::Data<Pool<ConnectionManager<PgConnection>>>,
    session: Session,
    payload: web::Json<LoginForm>,
) -> Result<HttpResponse, AppError> {
    let form = payload.into_inner();
    let possible_user = web::block(move || {
        let mut conn = pool.get().map_err(|e| {
            error!("Failed to get database connection: {}", e);
            AppError::InternalServerError
        })?;

        User::get_user_by_email(&mut conn, form.email).map_err(|e| {
            error!("Failed to get user: {}", e);
            AppError::DatabaseError
        })
    })
    .await??;

    match possible_user {
        Some(user) => {
            session
                .insert(
                    "user_data",
                    UserSession {
                        user_id: user.id.clone(),
                        role: Role::User,
                    },
                )
                .map_err(|e| {
                    error!("Troubles with session");
                    AppError::DatabaseError // TODO: i think need change this
                })?;
            Ok(HttpResponse::Ok().json("success"))
        }
        None => Ok(HttpResponse::Unauthorized().json("User not found")),
    }
}
