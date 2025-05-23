use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::user::User;

#[derive(Serialize)]
pub struct UserData {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub is_admin: bool,
}

#[derive(Debug, Deserialize)]
pub struct IssueForm {
    pub name: String,
    pub description: Option<String>,
    pub mitigation: Option<String>,
    pub cvss: Option<f64>,
    pub hosts: Vec<HostForm>,
}

#[derive(Debug, Deserialize)]
pub struct CreateIssueForm {
    pub name: String,
}

#[derive(Deserialize)]
pub struct ProofOfConceptForm {
    pub description: String,
    pub data: Vec<u8>,
    pub content_type: String,
    pub host: String,
}

#[derive(Deserialize, Debug)]
pub struct ProjectForm {
    pub name: String,
    pub description: Option<String>,
    pub scope: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub folder: String,
    pub team_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct HostForm {
    pub hostname: Option<String>,
    pub ip_address: String,
}

#[derive(Debug, MultipartForm)]
pub struct UploadReportTemplateForm {
    #[multipart(limit = "10MB")]
    pub file: Option<TempFile>,
    pub name: Text<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReportTemplateForm {
    pub file: Vec<u8>,
    pub filename: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct ReportForm {
    pub template_id: i32,
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
