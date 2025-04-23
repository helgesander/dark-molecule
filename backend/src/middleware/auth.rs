use actix_session::SessionExt;
use actix_web::body::MessageBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
use actix_web::Error;
use actix_web::middleware::Next;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub enum Role {
    Admin,
    User,
    Guest,
}


// TODO: fix to AppError later

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // Получаем сессию
    let session = req.get_session();

    // Проверяем авторизацию
    if session.get::<UserSession>("user_data").ok().flatten().is_none() {
        return Err(actix_web::error::ErrorUnauthorized("Invalid session"));
    }

    // Продолжаем обработку запроса
    next.call(req).await
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSession {
    pub(crate) user_id: Uuid,
    pub(crate) role: Role,
}
