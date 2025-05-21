use chrono::{DateTime, Utc};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("File error: {0}")]
    FileError(String),
    #[error("Template error: {0}")]
    TemplateError(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
}

pub struct Report {
    pub content: Vec<u8>,
    pub format: String,
    pub generated_at: DateTime<Utc>,
}

