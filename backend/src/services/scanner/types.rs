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
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Output parsing failed: {0}")]
    ParseError(String),
}

impl ScanStatus {
    pub fn is_completed(&self) -> bool {
        matches!(self, ScanStatus::Completed)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ScanResult {
    Nuclei(NucleiScanResult),
    Nmap(NmapScanResult),
    Gowitness(GowitnessScanResult),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NucleiScanResult {
    pub vulnerabilities: Vec<Vulnerability>,
    pub scan_time: f64,
    // ... другие поля специфичные для Nuclei
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NmapScanResult {
    pub hosts: Vec<Host>,
    pub scan_time: f64,
    // ... другие поля специфичные для Nmap
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GowitnessScanResult {
    pub screenshots: Vec<Screenshot>,
    pub scan_time: f64,
    // ... другие поля специфичные для Gowitness
}

// Вспомогательные структуры
#[derive(Debug, Serialize, Deserialize)]
pub struct Vulnerability {
    pub name: String,
    pub severity: String,
    pub description: String,
    // ...
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Host {
    pub ip: String,
    pub ports: Vec<Port>,
    // ...
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Port {
    pub number: u16,
    pub state: String,
    pub service: String,
    // ...
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Screenshot {
    pub url: String,
    pub path: String,
    pub status_code: u16,
    // ...
}
