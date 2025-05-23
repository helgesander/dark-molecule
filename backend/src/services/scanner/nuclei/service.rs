extern crate chrono;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use async_trait::async_trait;
use chrono::Utc;
use diesel::PgConnection;
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_json;
use thiserror::Error;
use uuid::Uuid;

use crate::models::scan::{NewScan, Scan};
use crate::services::scanner::types::Error;
use crate::services::scanner::{ScanStatus, VulnerabilityScanner};

#[derive(Clone)]
pub struct NucleiService {
    scans_dir: PathBuf,
    // nuclei_path: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NucleiScanRequest {
    pub target: String,
    pub templates: Option<Vec<String>>,
    pub severity: Option<Vec<String>>,
    pub output_format: Option<String>,
    // Другие параметры Nuclei
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NucleiScanResult {
    pub task_id: String,
    pub status: ScanStatus,
    pub findings: Option<Vec<NucleiFinding>>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NucleiFinding {
    pub template: String,
    pub severity: String,
    pub matched_at: String,
    // Другие поля из вывода Nuclei
}

impl NucleiService {
    pub fn new(scans_dir: impl AsRef<Path> /* nuclei_path: impl Into<String> */) -> Self {
        Self {
            scans_dir: scans_dir.as_ref().to_path_buf(),
        }
    }
}

#[async_trait]
impl VulnerabilityScanner for NucleiService {
    type ScanRequest = NucleiScanRequest;
    type ScanResult = NucleiScanResult;

    async fn create_scan(
        &self,
        conn: &mut PgConnection,
        project_id: Uuid,
        request: Self::ScanRequest,
    ) -> Result<String, Error> {
        let task_id = Uuid::new_v4().to_string();
        let output_dir = self.scans_dir.join(&task_id);
        fs::create_dir_all(&output_dir).map_err(|_| Error::IoError)?;

        let output_file = output_dir.join("results.json");
        let new_scan = NewScan {
            project_id,
            scanner_type: "nuclei".to_string(),
            status: "queued".to_string(),
            target: request.target,
            result_path: Some(output_file.to_str().ok_or(Error::IoError)?.to_string()),
        };

        Scan::create_scan(conn, new_scan).map_err(|e| {
            error!("Error creating scan: {}", e);
            Error::Database(e.to_string())
        })?;

        Ok(task_id)
    }

    async fn get_scan_result(&self, task_id: &str) -> Result<Self::ScanResult, Error> {
        let output_file = self.scans_dir.join(task_id).join("results.json");
        if !output_file.exists() {
            return Ok(NucleiScanResult {
                task_id: task_id.to_string(),
                status: ScanStatus::Running,
                findings: None,
                error: None,
            });
        }

        let content = fs::read_to_string(&output_file).map_err(|_| Error::IoError)?;

        let findings: Vec<NucleiFinding> =
            serde_json::from_str(&content).map_err(|e| Error::ParseError(e.to_string()))?;

        Ok(NucleiScanResult {
            task_id: task_id.to_string(),
            status: ScanStatus::Completed,
            findings: Some(findings),
            error: None,
        })
    }
}
