use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    username: String,
    email: String
}

impl User {
    pub fn new(username: String, email: String) -> User {
        User {
            id: Uuid::new_v4(),
            username,
            email
        }
    }
}