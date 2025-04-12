extern crate chrono;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use chrono::{DateTime, NaiveDateTime, Utc};
use log::{error, info};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::utils::errors::AppError;
use thiserror::Error;

#[derive(Clone)]
pub struct NucleiService {
    scans_dir: PathBuf,
    // nuclei_path: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanRequest {
    pub target: String,
    pub templates: Option<Vec<String>>,
    pub severity: Option<Vec<String>>,
    pub output_format: Option<String>,
    // Другие параметры Nuclei
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanResult {
    pub task_id: String,
    pub status: ScanStatus,
    pub findings: Option<Vec<NucleiFinding>>,
    pub error: Option<String>,
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ScanStatus {
    Queued,
    Running,
    Completed,
    Failed,
}


#[derive(Debug, Error)]
pub enum NucleiError {
    #[error("Nuclei execution failed: {0}")]
    ExecutionError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Output parsing failed: {0}")]
    ParseError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NucleiFinding {
    pub template: String,
    pub severity: String,
    pub matched_at: String,
    // Другие поля из вывода Nuclei
}

impl NucleiService {
    pub fn new(scans_dir: impl AsRef<Path>, /* nuclei_path: impl Into<String> */) -> Self {
        Self {
            scans_dir: scans_dir.as_ref().to_path_buf(),
            // nuclei_path: nuclei_path.into(),
        }
    }

    pub async fn create_scan(&self, request: ScanRequest) -> Result<String, NucleiError> { // TODO: read about it and change after maybe
        info!("Started Nuclei scan ");
        let task_id = Uuid::new_v4().to_string();
        let output_dir = self.scans_dir.join(&task_id);
        fs::create_dir_all(&output_dir).map_err(
            |e| NucleiError::IoError(e),
        )?;

        let scan_time = Utc::now().to_string().replace("-", "_");
        let output_file = output_dir.join(format!("result_{}.json", scan_time));

        // Запускаем сканирование в фоне
        tokio::task::spawn_blocking(move || {
            Self::run_nuclei_scan(request, &output_file)
        }).await
            .map_err(|e| NucleiError::ExecutionError(e.to_string()))??;

        Ok(task_id)
    }

    fn run_nuclei_scan(request: ScanRequest, output_file: &Path) -> Result<(), NucleiError> {
        let mut command = Command::new("nuclei");

        // Базовые параметры
        command
            .arg("-u")
            .arg(&request.target)
            .arg("-json")
            .arg(output_file);

        // Добавляем шаблоны, если указаны
        if let Some(templates) = request.templates {
            command.arg("-t");
            for template in templates {
                command.arg(template);
            }
        }

        // // Добавляем уровни серьезности
        // if let Some(severities) = request.severity {
        //     command.arg("-severity").arg(severities.join(",")); // TODO: maybe add later
        // }

        // Добавляем формат вывода, если указан
        if let Some(format) = request.output_format {
            command.arg("-output").arg(format);
        }

        let output = command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err( |e| NucleiError::IoError(e) )?;


        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr).into_owned();
            return Err(NucleiError::ExecutionError(error));
        }

        Ok(())
    }

    pub async fn get_scan_result(&self, task_id: &str) -> Result<ScanResult, NucleiError> {
        let output_file = self.scans_dir.join(task_id).join("results.json");

        if output_file.exists() {
            return Err(NucleiError::ParseError("Output file doesn't exist".to_string())) // TODO: change error for other error
        }
        Ok(ScanResult { // TODO: fix adding scan output to struct
            task_id: task_id.to_string(),
            status: ScanStatus::Running,
            findings: None,
            error: None,
        })
    }
}

