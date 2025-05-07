use gloo::net::http::Request;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use web_sys::RequestCredentials;
use std::sync::OnceLock;
use log;

#[derive(Debug, Clone, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub scope: Option<String>,
    pub team_id: Uuid,
    pub hosts: Vec<Host>,
    pub issues: Vec<Issue>,
    pub reports: Option<Vec<Report>>,
    pub services: Option<Vec<Service>>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Service {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Report {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProjectOverview {
    pub id: Uuid,
    pub name: String,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Team {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Host {
    pub id: Uuid,
    pub name: String,
    pub ip: String,
    pub os: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Issue {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub mitigation: Option<String>,
    pub cvss: f64,
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
    pub cvss: f64
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct CreateReportRequest {
    pub name: String,
    pub description: String,
    pub template: String,
}

#[derive(Serialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub scope: Option<String>,
    pub team_id: Uuid,
}

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub is_admin: bool,
    pub avatar: Option<String>
}

#[derive(Debug, Clone, Serialize)]
pub struct UserForm {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: String,
    pub email: String,
    pub password: String,
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
            .header("Content-Type", "application/json")
            .credentials(RequestCredentials::Include)
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

        response.json::<Vec<Team>>()
            .await
            .map_err(|e| format!("Ошибка при чтении ответа: {}", e))
    }

    pub async fn create_project(
        &self,
        name: String,
        scope: Option<String>,
        team_id: Uuid,
    ) -> Result<Project, String> {
        let request = CreateProjectRequest {
            name,
            scope,
            team_id,
        };

        let response = Request::post(&format!("{}/project/", self.base_url))
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

    pub async fn get_project(&self, id: Uuid) -> Result<Project, String> {
        let response = Request::get(&format!("{}/project/{}", self.base_url, id))
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

        response.json::<Project>()
            .await
            .map_err(|e| format!("Ошибка при чтении ответа: {}", e))
    }

    pub async fn get_project_hosts(&self, project_id: Uuid) -> Result<Vec<Host>, String> {
        let response = Request::get(&format!("{}/project/{}/hosts", self.base_url, project_id))
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

        response.json::<Vec<Host>>()
            .await
            .map_err(|e| format!("Ошибка при чтении ответа: {}", e))
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

    pub async fn get_user(&self, id: &String) -> Result<User, String> {
        let response = Request::get(&format!("{}/user/{}", self.base_url, id))
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

    pub async fn create_report(&self, project_id: Uuid, report_name: String, report_description: String, report_template: String) -> Result<Report, String> {
        let request = CreateReportRequest {
            name: report_name,
            description: report_description,
            template: report_template,
        };
        
        let response = Request::post(&format!("{}/project/{}/report/", self.base_url, project_id))
            .header("Content-Type", "application/json")
            .credentials(RequestCredentials::Include)
            .send()
            .await
            .map_err(|e| format!("Ошибка при отправке запроса: {}", e))?;

        if !response.ok() {
            return Err("Ошибка при создании отчета".to_string());
        }

        response.json::<Report>()
            .await
            .map_err(|e| format!("Ошибка при чтении ответа: {}", e))
    }

    pub async fn get_report(&self, project_id: Uuid, report_id: Uuid) -> Result<Report, String> {
        let response = Request::get(&format!("{}/project/{}/report/{}/", self.base_url, project_id, report_id)) // TODO: chande path to files
            .header("Content-Type", "application/json")
            .credentials(RequestCredentials::Include)
            .send()
            .await
            .map_err(|e| format!("Ошибка при отправке запроса: {}", e))?;

        if !response.ok() {
            return Err(format!("Ошибка сервера: {}", response.status()));
        }   

        response.json::<Report>()
            .await
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

    pub async fn get_issue(&self, project_id: Uuid, issue_id: Uuid) -> Result<Issue, String> {
        let response = Request::get(&format!("{}/project/{}/issue/{}", self.base_url, project_id, issue_id))
            .header("Content-Type", "application/json")
            .credentials(RequestCredentials::Include)
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
}   