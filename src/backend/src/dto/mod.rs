use serde::{Serialize, Deserialize};
use crate::utils::Role;

#[derive(Serialize)]
pub struct CreateUserResponseDto {
    username: String,
    roles: Vec<String>,
    company: String,
}

#[derive(Deserialize)]
pub struct CreateUserRequestDto {
    username: String,
    email: String,
    password: String,
    company: String,
}

impl CreateUserRequestDto {
    pub fn get_username(&self) -> String {
        self.username.clone()
    }

    // TODO: change maybe (clone wtf)
    pub fn get_company(&self) -> String {
        self.company.clone()
    }
}

impl CreateUserResponseDto {
    pub fn new(username: String, company: String) -> CreateUserResponseDto {
        CreateUserResponseDto {
            username,
            roles: vec![Role::User.to_string()],
            company
        }
    }
}