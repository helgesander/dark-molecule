use std::sync::Arc;
use std::{env, fs};

use actix_web::cookie::Key;
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Clone)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub log_level: String,
    pub secret_key: Key,
    pub templates_path: String,
    pub scans_path: String,
    pub reports_path: String,
}

impl AppConfig {
    pub fn new() -> Self {
        let templates_path = env::var("TEMPLATES_PATH").unwrap_or("/app/templates/".into());
        let scans_path = env::var("SCANS_PATH").unwrap_or("/app/scans/".into());
        let reports_path = env::var("REPORTS_PATH").unwrap_or("/app/reports/".into());

        Self::create_dirs_if_doesnt_exist(
            templates_path.clone(),
            scans_path.clone(),
            reports_path.clone(),
        )
        .unwrap();

        Self {
            database: DatabaseConfig {
                url: env::var("DATABASE_URL").unwrap_or_else(|_| {
                    "postgres://postgres:postgres@localhost:5432/dark_molecule".to_string()
                }),
                max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(5), // TODO: change
            },
            server: ServerConfig {
                host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: env::var("SERVER_PORT")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(8000),
            },
            log_level: env::var("LOG_LEVEL").unwrap_or("info".into()),
            secret_key: Key::generate(),
            templates_path,
            scans_path,
            reports_path,
        }
    }

    fn create_dirs_if_doesnt_exist(
        templates: String,
        scans: String,
        reports: String,
    ) -> std::io::Result<()> {
        fs::create_dir_all(&templates)?;
        fs::create_dir_all(&scans)?;
        fs::create_dir_all(&reports)
    }
}

pub static CONFIG: Lazy<Arc<AppConfig>> = Lazy::new(|| Arc::new(AppConfig::new()));
