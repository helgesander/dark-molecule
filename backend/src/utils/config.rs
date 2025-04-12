use std::env;
use actix_web::cookie::Key;
use dotenv::dotenv;

#[derive(Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub port: u16,
    pub log_level: String,
    pub secret_key: Key,
    pub templates_path: String,
    pub scans_path: String
}

impl AppConfig {
    pub fn new() -> Result<Self, anyhow::Error> {
        dotenv().ok();
        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
            port: env::var("PORT")?.parse()?,
            log_level: env::var("LOG_LEVEL").unwrap_or("info".into()),
            secret_key: Key::generate(),
            templates_path: env::var("TEMPLATES_PATH").unwrap_or("/app".into()),
            scans_path: env::var("SCANS_PATH").unwrap_or("/app/".into()),
        })
    }
}