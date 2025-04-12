use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use actix_session::SessionExt;
use actix_web::body::MessageBody;
use crate::utils::errors::AppError;

#[derive(Serialize, Deserialize, Debug)]
pub enum Role {
    Admin,
    User,
    Guest,
}

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>
) -> Result<ServiceResponse<impl MessageBody>, AppError> {
    let session = req.get_session();
    let user_data = session.get::<UserSession>("user_data").ok();

    match user_data {
        Some(_) => Ok(next.call(req).await.map_err(|_| {AppError::InternalServerError})?),
        None => Err(AppError::UnauthorizedError)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSession {
    pub(crate) user_id: Uuid,
    pub(crate) role: Role
}