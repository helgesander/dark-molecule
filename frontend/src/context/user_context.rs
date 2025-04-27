use yew::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub is_admin: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UserContext {
    pub user: Option<User>,
    pub set_user: Callback<Option<User>>,
}

impl UserContext {
    pub fn new(user: Option<User>, set_user: Callback<Option<User>>) -> Self {
        Self { user, set_user }
    }
}

pub fn use_user_context() -> UserContext {
    use_context::<UserContext>().expect("UserContext not found")
} 