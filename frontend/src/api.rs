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
    pub title: String,
    pub description: String,
    pub severity: String,
    pub status: String,
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
    pub avatar: Option<String>,
}

#[derive(Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginResponse {
    pub user: User,
}

pub struct ApiClient {
    base_url: String,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            base_url: "http://localhost:8000/api".to_string(),
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

    pub async fn register(&self, username: String, email: String, password: String) -> Result<User, String> {
        let response = Request::post(&format!("{}/user/", self.base_url))
            .header("Content-Type", "application/json")
            .credentials(RequestCredentials::Include)
            .json(&RegisterRequest { username, email, password })
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
}