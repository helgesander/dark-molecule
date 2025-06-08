use actix_session::SessionExt;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use actix_web::Error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::errors::AppError;

#[derive(Serialize, Deserialize, Debug)]
pub enum Role {
    Admin,
    User,
    Guest,
}


pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let session = req.get_session();

    if session
        .get::<UserSession>("user_data")
        .ok()
        .flatten()
        .is_none()
    {
        return Err(AppError::UnauthorizedError.into());
    }

    next.call(req)
        .await
        .map_err(|_| AppError::InternalServerError.into())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSession {
    pub(crate) user_id: Uuid,
    pub(crate) role: Role,
}
