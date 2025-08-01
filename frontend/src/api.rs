use gloo::net::http::Request;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use web_sys::RequestCredentials;
use std::sync::OnceLock;
use chrono::{NaiveDate, Utc};
use gloo::console::warn;
use log;
use crate::debug_log;

#[derive(Debug, Clone, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub scope: Option<String>,
    pub team_id: Uuid,
    pub hosts: Vec<Host>,
    pub issues: Vec<Issue>,
    pub reports: Option<Vec<ReportPreview>>,
    pub services: Option<Vec<Service>>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Service {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ScanRequest {
    pub r#type: String,
    pub target: String
}

#[derive(Serialize, Deserialize)]
pub struct ScanResponse {
    pub scan_id: String,
    pub status: String
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Report {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ReportData {
    pub filename: String,
    pub data: Vec<u8>
}

#[derive(Deserialize)]
pub struct ReportTemplatePreview {
    pub id: i32,
    pub name: String,
    pub extension: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProjectOverview {
    pub id: Uuid,
    pub name: String,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TeamForm {
    pub name: String,
    pub description: Option<String>,
    pub admin_id: Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Team {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub admin_id: Uuid,
}


#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Host {
    pub id: i32,
    pub hostname: Option<String>,
    pub ip_address: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CreateHostRequest {
    pub hostname: Option<String>,
    pub ip_address: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Issue {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub mitigation: Option<String>,
    pub cvss: f64,
    pub hosts: Vec<Host>
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct IssueFullResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub mitigation: Option<String>,
    pub cvss: f64,
    pub hosts: Vec<Host>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CreateIssueRequest {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct UpdateIssue {
    pub name: String,
    pub description: Option<String>,
    pub mitigation: Option<String>,
    pub cvss: f64,
    pub hosts: Vec<Host>
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct ReportPreview {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CreateReportRequest {
    pub template_id: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub file: Vec<u8>
}

#[derive(Serialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub scope: Option<String>,
    pub team_id: Uuid,
    pub folder: String,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub id: Option<Uuid>,
    pub username: String,
    pub email: String,
    pub is_admin: bool,
    pub avatar: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserForm {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub is_admin: Option<bool>,
}

#[derive(Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginResponse {
    pub user: User
}

pub struct ApiClient {
    base_url: String,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            base_url: "http://localhost:8000/api".to_string(), // TODO: add it to AppState 
        }
    }

    pub fn get() -> &'static Self {
        static INSTANCE: OnceLock<ApiClient> = OnceLock::new();
        INSTANCE.get_or_init(ApiClient::new)
    }

    pub async fn get_projects(&self) -> Result<Vec<ProjectOverview>, String> {
        log::info!("Fetching projects from {}", format!("{}/project/", self.base_url));
        
        let response = Request::get(&format!("{}/project/", self.base_url))
            .send()
            .await
            .map_err(|e| {
                log::error!("Error sending request: {}", e);
                format!("Ошибка при отправке запроса: {}", e)
            })?;

        log::info!("Response status: {}", response.status());

        if response.status() == 401 {
            log::warn!("Unauthorized access");
            return Err("unauthorized".to_string());
        }

        if !response.ok() {
            let status = response.status();
            log::error!("Server error: {}", status);
            return Err(format!("Ошибка сервера: {}", status));
        }

        let projects = response.json::<Vec<ProjectOverview>>()
            .await
            .map_err(|e| {
                log::error!("Error parsing response: {}", e);
                format!("Ошибка при чтении ответа: {}", e)
            })?;

        log::info!("Successfully fetched {} projects", projects.len());
        Ok(projects)
    }


    pub async fn logout(&self) -> Result<(), String> {
        let response = Request::post(&format!("{}/auth/logout", self.base_url))
            .header("Content-Type", "application/json")
            .credentials(RequestCredentials::Include)
            .send()
            .await
            .map_err(|e| format!("Ошибка при отправке запроса: {}", e))?;

        if !response.ok() {
            return Err(format!("Ошибка сервера: {}", response.status()));
        }

        Ok(())
    }

    pub async fn get_teams(&self) -> Result<Vec<Team>, String> {
        let response = Request::get(&format!("{}/team/", self.base_url))
            .send()
            .await
            .map_err(|e| format!("Ошибка при отправке запроса: {}", e))?;

        if response.status() == 401 {
            return Err("unauthorized".to_string());
        }

        if !response.ok() {
            return Err(format!("Ошибка сервера: {}", response.status()));
        }

        response.json::<Vec<Team>>() 
            .await
            .map_err(|e| format!("Ошибка при чтении ответа: {}", e))
    }

    pub async fn create_project(
        &self,
        project: &CreateProjectRequest,
    ) -> Result<Project, String> {


        let response = Request::post(&format!("{}/project/", self.base_url))
            .json(&project)
            .unwrap()
            .send()
            .await
            .map_err(|e| format!("Ошибка при отправке запроса: {}", e))?;

        if !response.ok() {
            return Err(format!("Ошибка сервера: {}", response.status()));
        }

        response.json::<Project>()
            .await
            .map_err(|e| format!("Ошибка при чтении ответа: {}", e))
    }

    pub async fn login(&self, email: String, password: String) -> Result<User, String> {
        let login_request = LoginRequest { email, password };
        
        let response = Request::post(&format!("{}/auth/", self.base_url))
            .header("Content-Type", "application/json")
            .credentials(RequestCredentials::Include)
            .json(&login_request)
            .unwrap()
            .send()
            .await
            .map_err(|e| format!("Ошибка при отправке запроса: {}", e))?;

        if !response.ok() {
            return Err("Неверный email или пароль".to_string());
        }

        if response.status() == 401 {
            return Err("Неверный email или пароль".to_string());
        }

        let login_response = response.json::<LoginResponse>()
            .await
            .map_err(|e| format!("Ошибка при чтении ответа: {}", e))?;

        Ok(login_response.user)
    }

    pub async fn delete_host(&self, project_id: Uuid, host_id: i32) -> Result<(), String> {
        let response = Request::delete(&format!("{}/project/{}/host/{}", self.base_url, project_id, host_id))
            .send()
            .await
            .map_err(|e| format!("Ошибка при отправке запроса: {}", e))?;

        if response.status() == 401 {
            return Err("unauthorized".to_string());
        }

        if !response.ok() {
            return Err(format!("Ошибка сервера: {}", response.status()));
        }

        Ok(())
    }

    pub async fn delete_issue(&self, project_id: Uuid, issue_id: Uuid) -> Result<(), String> {
        let response = Request::delete(&format!("{}/project/{}/issue/{}", self.base_url, project_id, issue_id))
            .send()
            .await
            .map_err(|e| format!("Ошибка при отправке запроса: {}", e))?;

        if response.status() == 401 {
            return Err("unauthorized".to_string());
        }

        if !response.ok() {
            return Err(format!("Ошибка сервера: {}", response.status()));
        }

        Ok(())

    }

    pub async fn get_project_issues(&self, project_id: Uuid) -> Result<Vec<Issue>, String> {
        let response = Request::get(&format!("{}/project/{}/issues", self.base_url, project_id))
            .header("Content-Type", "application/json")
            .credentials(RequestCredentials::Include)
            .send()
            .await
            .map_err(|e| format!("Ошибка при отправке запроса: {}", e))?;

        if response.status() == 401 {
            return Err("unauthorized".to_string());
        }

        if !response.ok() {
            return Err(format!("Ошибка сервера: {}", response.status()));
        }

        response.json::<Vec<Issue>>()
            .await
            .map_err(|e| format!("Ошибка при чтении ответа: {}", e))
    }

    pub async fn get_full_project(&self, project_id: Uuid) -> Result<Project, String> {
        let response = Request::get(&format!("{}/project/{}", self.base_url, project_id))
            .header("Content-Type", "application/json")
            .credentials(RequestCredentials::Include)
            .send()
            .await
            .map_err(|e| format!("Ошибка при отправке запроса: {}", e))?;

        if response.status() == 401 {
            return Err("unauthorized".to_string());
        }

        if response.status() == 404 {
            return Err("project not found".to_string());
        }

        if !response.ok() {
            return Err(format!("Ошибка сервера: {}", response.status()));
        }

        response.json::<Project>()
            .await
            .map_err(|e| format!("Ошибка при чтении ответа: {}", e))
    }

    pub async fn get_user(&self, id: &Option<Uuid>) -> Result<User, String> {
        let url = format!("{}/user/{}", self.base_url, id.as_ref().ok_or("No user ID provided")?);
        let response = Request::get(&url)
            .header("Content-Type", "application/json")
            .credentials(RequestCredentials::Include)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if response.status() == 200 {
            let mut user = response.json::<User>()
                .await
                .map_err(|e| e.to_string())?;
            // Ensure we preserve the ID from the request
            user.id = *id;
            Ok(user)
        } else {
            Err("Failed to get user".to_string())
        }
    }

    pub async fn get_users(&self, size: Option<u32>, name: Option<String>) -> Result<Vec<User>, String> {
        let mut query_params = Vec::new();
        
        if let Some(s) = size {
            query_params.push(format!("size={}", s));
        }
        
        if let Some(n) = name {
            query_params.push(format!("name={}", n));
        }
        
        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };

        let response = Request::get(&format!("{}/user/{}", self.base_url, query_string))
            .header("Content-Type", "application/json")
            .credentials(RequestCredentials::Include)
            .send()
            .await
            .map_err(|e| format!("Ошибка при отправке запроса: {}", e))?;

        if response.status() == 401 {
            return Err("unauthorized".to_string());
        }

        if !response.ok() {
            return Err(format!("Ошибка сервера: {}", response.status()));
        }

        response.json::<Vec<User>>()
            .await
            .map_err(|e| format!("Ошибка при чтении ответа: {}", e))
    }

    pub async fn create_user(&self, form: &UserForm) -> Result<User, String> {
        let response = Request::post(&format!("{}/user/", self.base_url))
            .header("Content-Type", "application/json")
            .credentials(RequestCredentials::Include)
            .json(form)
            .unwrap()
            .send()
            .await
            .map_err(|e| format!("Ошибка при отправке запроса: {}", e))?;

        if !response.ok() {
            return Err(format!("Ошибка сервера: {}", response.status()));
        }   

        response.json::<User>()
            .await
            .map_err(|e| format!("Ошибка при чтении ответа: {}", e))
    }

    pub async fn register(&self, form: &UserForm) -> Result<User, String> {
        let response = Request::post(&format!("{}/user/", self.base_url))
            .header("Content-Type", "application/json")
            .credentials(RequestCredentials::Include)
            .json(form)
            .unwrap()
            .send()
            .await
            .map_err(|e| format!("Ошибка при отправке запроса: {}", e))?;

        if !response.ok() {
            return Err(format!("Ошибка сервера: {}", response.status()));
        }

        response.json::<User>()
            .await
            .map_err(|e| format!("Ошибка при чтении ответа: {}", e))
    }

    pub async fn create_report(&self, project_id: Uuid, report_template_id: i32) -> Result<ReportData, String> {
        let request = CreateReportRequest {
            template_id: report_template_id,
        };
        
        let response = Request::post(&format!("{}/project/{}/report", self.base_url, project_id))
            .header("Content-Type", "application/json")
            .credentials(RequestCredentials::Include)
            .json(&request)
            .unwrap()
            .send()
            .await
            .map_err(|e| format!("Ошибка при отправке запроса: {}", e))?;

        if !response.ok() {
            return Err("Ошибка при создании отчета".to_string());
        }

        let content_disposition = response.headers()
            .get("Content-Disposition")
            .unwrap_or_default();

        let filename = content_disposition
            .split("filename=")
            .nth(1)
            .map(|s| s.trim_matches('"').to_string())
            .unwrap_or_else(|| format!("report_{}.bin", project_id));

        // Получаем бинарные данные
        let data = response.binary()
            .await
            .map_err(|e| format!("Ошибка при чтении бинарных данных: {}", e))?;

        Ok(ReportData {
            filename,
            data
        })
    }


    pub async fn get_reports_preview(&self, project_id: Uuid) -> Result<Vec<ReportPreview>, String> {
        let response = Request::get(&format!("{}/project/{}/report/all", self.base_url, project_id))
        .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| format!("Ошибка при отправке запроса: {}", e))?;

        if !response.ok() {
            return Err(format!("Ошибка сервера: {}", response.status()));
        }


        response.json::<Vec<ReportPreview>>().await
            .map_err(|e| format!("Ошибка при чтении ответа: {}", e))

    }

    pub async fn create_issue(&self, project_id: Uuid, name: String) -> Result<Issue, String> {
        let request = CreateIssueRequest {
            name,
        };
        let response = Request::post(&format!("{}/project/{}/issue", self.base_url, project_id))
            .header("Content-Type", "application/json")
            .credentials(RequestCredentials::Include)
            .json(&request)
            .unwrap()
            .send()
            .await
            .map_err(|e| format!("Ошибка при отправке запроса: {}", e))?;

        if !response.ok() {
            return Err(format!("Ошибка сервера: {}", response.status()));
        }
        
        response.json::<Issue>()
            .await
            .map_err(|e| format!("Ошибка при чтении ответа: {}", e))    
    }

    pub async fn get_issue(&self, project_id: Uuid, issue_id: Uuid) -> Result<IssueFullResponse, String> {
        let response = Request::get(&format!("{}/project/{}/issue/{}", self.base_url, project_id, issue_id))
            .header("Content-Type", "application/json")
            .credentials(RequestCredentials::Include)
            .send()
            .await
            .map_err(|e| format!("Ошибка при отправке запроса: {}", e))?;

        if !response.ok() {
            return Err(format!("Ошибка сервера: {}", response.status()));
        }   

        response.json::<IssueFullResponse>()
            .await
            .map_err(|e| format!("Ошибка при чтении ответа: {}", e))
    }

    pub async fn edit_issue(&self, project_id: Uuid, issue_id: Uuid, issue: UpdateIssue) -> Result<Issue, String> {

        let response = Request::put(&format!("{}/project/{}/issue/{}", self.base_url, project_id, issue_id))
            .header("Content-Type", "application/json")
            .credentials(RequestCredentials::Include)
            .json(&issue)
            .unwrap()
            .send()
            .await
            .map_err(|e| format!("Ошибка при отправке запроса: {}", e))?;

        if !response.ok() {
            return Err(format!("Ошибка сервера: {}", response.status()));
        }

        response.json::<Issue>()
            .await
            .map_err(|e| format!("Ошибка при чтении ответа: {}", e))
    }

    pub async fn create_team(&self, name: String, description: Option<String>, admin_id: Uuid) -> Result<Team, String> {
        let form = TeamForm {
            name,
            description,
            admin_id,
        };

        let response = Request::post(&format!("{}/team/", self.base_url))
            .header("Content-Type", "application/json")
            .credentials(RequestCredentials::Include)
            .json(&form)
            .unwrap()
            .send()
            .await
            .map_err(|e| format!("Ошибка при отправке запроса: {}", e))?;

        if !response.ok() {
            return Err(format!("Ошибка сервера: {}", response.status()));
        }

        response.json::<Team>()
            .await
            .map_err(|e| format!("Ошибка при чтении ответа: {}", e))
    }

    pub async fn get_hosts(&self, project_id: Uuid) -> Result<Vec<Host>, String> {
        let response = Request::get(&format!("{}/project/{}/hosts", self.base_url, project_id))
            .send()
            .await
            .map_err(|e| format!("Ошибка при отправке запроса: {}", e))?;

        if !response.ok() {
            return Err(format!("Ошибка сервера: {}", response.status()));
        }

        response.json::<Vec<Host>>()
            .await
            .map_err(|e| format!("Ошибка при чтении ответа: {}", e))
    }
    

    pub async fn create_host(&self, project_id: Uuid, host: CreateHostRequest) -> Result<Host, String> {
        let response = Request::post(&format!("{}/project/{}/host", self.base_url, project_id))
            .json(&host)
            .unwrap()
            .send()
            .await
            .map_err(|e| format!("Ошибка при отправке запроса: {}", e))?;
    
        if !response.ok() {
            return Err(format!("Ошибка сервера: {}", response.status()));
        }
    
        response.json::<Host>()
            .await
            .map_err(|e| format!("Ошибка при чтении ответа: {}", e))
    }

    pub async fn update_host(&self, project_id: Uuid, host: CreateHostRequest) -> Result<Host, String> {
        let response = Request::put(&format!("{}/project/{}/host", self.base_url, project_id))
            .json(&host)
            .unwrap()
            .send()
            .await
            .map_err(|e| format!("Ошибка при отправке запроса: {}", e))?;
    
        if !response.ok() {
            return Err(format!("Ошибка сервера: {}", response.status()));
        }
    
        response.json::<Host>()
            .await
            .map_err(|e| format!("Ошибка при чтении ответа: {}", e))
    }

    pub async fn get_report_templates(&self) -> Result<Vec<ReportTemplatePreview>, String> {
        let response = Request::get(&format!("{}/templates/all", self.base_url))
            .send()
            .await
            .map_err(|e| format!("Ошибка при отправке запроса: {}", e))?;

        if !response.ok() {
            return Err(format!("Ошибка сервера: {}", response.status()));
        };

        response.json::<Vec<ReportTemplatePreview>>()
        .await
            .map_err(|e| format!("Ошибка при чтении ответа: {}", e))
    }

    pub async fn create_report_template(&self, form: web_sys::FormData) -> Result<(), String> {
        let response = Request::post(&format!("{}/template/", self.base_url))
            .body(&form)
            .unwrap()
            .send()
            .await
            .map_err(|e| format!("Ошибка при отправке запроса: {}", e))?;

        if !response.ok() {
            return Err(format!("Ошибка сервера: {}", response.status()));
        }

        Ok(())
    }

    pub async fn download_report(&self, project_id: Uuid, report_id: i32) -> Result<ReportData, String> {
        let response = Request::get(&format!("{}/project/{}/report/{}", self.base_url, project_id, report_id))
            .send()
            .await
            .map_err(|e| format!("Ошибка при отправке запроса: {}", e))?;

        if !response.ok() {
            return Err(format!("Ошибка сервера: {}", response.status()));
        }

        let content_disposition = response.headers()
            .get("content-disposition")
            .unwrap_or_default();
        debug_log!("Content-Disposition: {}", content_disposition);

        let filename =
                content_disposition.split("filename=")
                    .nth(1)
                    .map(|s| s.trim_matches('"').to_string())
            .unwrap_or_else(|| {
                warn!("Content-Disposition header missing or malformed, using default filename");
                format!("report_{}.md", Utc::now().format("%Y%m%d_%H%M%S"))
            });

        // Получаем бинарные данные
        let data = response.binary()
            .await
            .map_err(|e| format!("Ошибка при чтении бинарных данных: {}", e))?;

        Ok(ReportData {
            filename,
            data
        })
    }

    pub async fn create_scan(&self, project_id: Uuid, request: &ScanRequest) -> Result<ScanResponse, String> {
        let response = Request::post(&format!("{}/project/{}/scan", self.base_url, project_id))
            .json(&request)
            .unwrap()
            .send()
            .await
            .map_err(|e| format!("Ошибка при отправке запроса: {}", e))?;

        if response.status() == 401 {
            return Err("unauthorized".to_string())
        }

        if !response.ok() {
            return Err(format!("Ошибка сервера: {}", response.status()));
        }

        response.json::<ScanResponse>()
            .await
            .map_err(|e| format!("Ошибка при чтении ответа: {}", e))
    }

}

