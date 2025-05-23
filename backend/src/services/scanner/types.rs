use quick_xml::events::attributes::AttrError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScanStatus {
    Queued,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Scanner execution failed: {0}")]
    ExecutionError(String),
    #[error("I/O Error: {0}")]
    IoError(String),
    #[error("Output parsing failed: {0}")]
    ParseError(String),
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    #[error("Database error: {0}")]
    Database(String),
    #[error("Scan not found: {0}")]
    NotFound(String),
}

impl From<quick_xml::Error> for Error {
    fn from(e: quick_xml::Error) -> Self {
        Error::ParseError(e.to_string())
    }
}

impl From<quick_xml::DeError> for Error {
    fn from(e: quick_xml::DeError) -> Self {
        Error::ParseError(e.to_string())
    }
}

impl ScanStatus {
    pub fn is_completed(&self) -> bool {
        matches!(self, ScanStatus::Completed)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vulnerability {
    pub name: String,
    pub severity: String,
    pub description: String,
}
