use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::models::user::User;

#[derive(Serialize, Deserialize)]
pub struct Company {
    pub id: Uuid,
    pub name: String,
    pub employees: Vec<User>,
}

impl Company {
    pub fn new(name: String) -> Company {
        Company {
            id: Uuid::new_v4(),
            name,
            employees: Vec::new()
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_employees(&self) -> Vec<User> {
        self.employees.clone()
    }
}