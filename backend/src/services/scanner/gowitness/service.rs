use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::services::scanner::{VulnerabilityScanner, ScanStatus};
use crate::services::scanner::types::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GowitnessScanRequest {
    pub target: String,
    pub options: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GowitnessScanResult {
    pub task_id: String,
    pub status: ScanStatus,
    pub findings: Option<Vec<String>>,
    pub error: Option<String>,
}

pub struct GowitnessService {
    scans_dir: PathBuf,
}

impl GowitnessService {
    pub fn new(scans_dir: impl AsRef<Path>) -> Self {
        Self { scans_dir: scans_dir.as_ref().to_path_buf() }
    }
}

#[async_trait]
impl VulnerabilityScanner for GowitnessService {
    type ScanRequest = GowitnessScanRequest;
    type ScanResult = GowitnessScanResult;

    async fn create_scan(&self, request: Self::ScanRequest) -> Result<String, Error> {
        let task_id = Uuid::new_v4().to_string();
        let output_dir = self.scans_dir.join(&task_id);
        std::fs::create_dir_all(&output_dir).map_err(|e| Error::IoError(e))?;
        
        let output_file = output_dir.join("screenshot.png");
        self.run_scan(request, &output_file)?;
        
        Ok(task_id)
    }

    async fn get_scan_result(&self, task_id: &str) -> Result<Self::ScanResult, Error> {
        let output_file = self.scans_dir.join(task_id).join("screenshot.png");
        if !output_file.exists() {
            return Ok(GowitnessScanResult {
                task_id: task_id.to_string(),
                status: ScanStatus::Running,
                findings: None,
                error: None,
            });
        }
        
        Ok(GowitnessScanResult {
            task_id: task_id.to_string(),
            status: ScanStatus::Completed,
            findings: Some(vec![output_file.to_string_lossy().into_owned()]),
            error: None,
        })
    }

    fn run_scan(
        &self,
        request: Self::ScanRequest,
        output_file: &Path,
    ) -> Result<Self::ScanResult, Error> {
        let mut command = Command::new("gowitness");
        
        command
            .arg("single")
            .arg("-u")
            .arg(&request.target)
            .arg("-o")
            .arg(output_file);
            
        if let Some(options) = request.options {
            command.arg(&options);
        }
        
        let output = command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| Error::IoError(e))?;
            
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr).into_owned();
            return Err(Error::ExecutionError(error));
        }
        
        Ok(GowitnessScanResult {
            task_id: Uuid::new_v4().to_string(),
            status: ScanStatus::Running,
            findings: None,
            error: None,
        })
    }
}


