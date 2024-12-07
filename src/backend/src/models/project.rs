use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::models::user::User;

#[derive(Serialize, Deserialize)]
pub struct Project {
    id: Uuid,
    name: String,
    author: User
}

impl Project {
    pub fn new(name: String, author: User) -> Project {
        Project {
            id: Uuid::new_v4(),
            name,
            author
        }
    }

}