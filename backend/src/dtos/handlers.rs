use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::project::Project;
use crate::models::user::User;
// use crate::db::schema::projects::;


#[derive(Serialize)]
pub struct UserData {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub is_admin: bool,
}

#[derive(Serialize)]
pub struct ProjectResponse {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub struct IssueForm {
    pub name: String,
    pub description: Option<String>,
    pub mitigation: Option<String>,
    pub cvss: Option<f64>,
}

#[derive(Deserialize)]
pub struct ProofOfConceptForm {
    description: String,
    data: Vec<u8>,
}

#[derive(Deserialize, Debug)]
pub struct ProjectForm {
    pub name: String,
    pub description: Option<String>,
    pub scope: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub folder: String,
    pub team_id: Uuid
}

#[derive(Deserialize)]
pub struct HostForm {
    pub hostname: Option<String>,
    pub ip_address: String,
}



impl UserData {
    pub fn new(data: &User) -> Option<UserData> {
        let first_name = data.first_name.as_ref().map(|s| s.to_string());
        let last_name = data.last_name.as_ref().map(|s| s.to_string());

        Some(UserData {
            first_name: first_name.unwrap_or_else(|| "".to_string()),
            last_name: last_name.unwrap_or_else(|| "".to_string()),
            username: data.username.clone(),
            email: data.email.clone(),
            is_admin: data.is_admin,
        })
    }
}

impl ProjectResponse {
    pub fn new(data: &Project) -> ProjectResponse {
        if let Some(description) = data.description.clone() {
            ProjectResponse {
                name: data.name.clone(),
                description,
            }
        } else {
            ProjectResponse {
                name: data.name.clone(),
                description: String::new(),
            }
        }
    }
}