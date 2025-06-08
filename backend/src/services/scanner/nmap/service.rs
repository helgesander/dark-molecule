use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use actix_web::guard::Host;
use async_trait::async_trait;
use diesel::PgConnection;
use log::{debug, error};
use quick_xml::de::from_str;
use quick_xml::DeError;
use serde::{Deserialize, Serialize};
use tokio::process::Command as AsyncCommand;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::models::host::HostResponse;
use crate::models::scan::{NewScan, Scan, UpdateScan};
use crate::services::scanner::types::{AnyScanResult, Error};
use crate::services::scanner::VulnerabilityScanner;
use crate::db::Pool;
use crate::db::schema::hosts::project_id;
use crate::dtos::handlers::HostForm;
use crate::models;
use crate::utils::errors::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScanStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
}

impl From<ScanStatus> for String {
    fn from(status: ScanStatus) -> String {
        match status {
            ScanStatus::Pending => "pending".to_string(),
            ScanStatus::Running => "running".to_string(),
            ScanStatus::Completed => "completed".to_string(),
            ScanStatus::Failed(_) => "failed".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanState {
    pub status: ScanStatus,
    pub result: Option<NmapScanResult>,
}

#[derive(Debug, Deserialize)]
struct Address {
    #[serde(rename = "@addr")]
    addr: String,
    #[serde(rename = "@addrtype")]
    addr_type: String,
}

#[derive(Debug, Deserialize)]
struct Status {
    #[serde(rename = "@state")]
    state: String,
}

#[derive(Debug, Deserialize)]
struct Host {
    #[serde(rename = "status")]
    status: Status,
    #[serde(rename = "address", default)]
    addresses: Vec<Address>,
}

#[derive(Debug, Deserialize)]
struct HostHint {
    #[serde(rename = "status")]
    status: Status,
    #[serde(rename = "address", default)]
    addresses: Vec<Address>,
}

#[derive(Debug, Deserialize)]
struct NmapRun {
    #[serde(rename = "host", default)]
    hosts: Vec<Host>,
    #[serde(rename = "hosthint", default)]
    host_hints: Vec<HostHint>,
}

#[derive(Clone)]
pub struct NmapService {
    scans_dir: PathBuf,
    active_scans: Arc<Mutex<HashMap<String, ScanState>>>,
    pool: Pool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NmapScanRequest {
    pub target: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NmapScanResult {
    pub output_file: String,
    pub hosts: Vec<String>,
}

impl NmapService {
    pub fn new(scans_dir: impl AsRef<Path>, pool: Pool) -> Self {
        Self {
            scans_dir: scans_dir.as_ref().to_path_buf(),
            active_scans: Arc::new(Mutex::new(HashMap::new())),
            pool,
        }
    }

    pub fn parse_up_hosts(xml_content: &str) -> Result<Vec<String>, DeError> {
        let nmap_run: NmapRun = from_str(xml_content)?;

        let mut up_hosts = Vec::new();

        // Обрабатываем обычные хосты
        for host in nmap_run.hosts {
            if host.status.state == "up" {
                for addr in host.addresses {
                    if addr.addr_type == "ipv4" || addr.addr_type == "ipv6" {
                        up_hosts.push(addr.addr);
                    }
                }
            }
        }
        Ok(up_hosts)
    }

    async fn update_scan_status(
        &self,
        scan_id: Uuid,
        status: ScanStatus,
    ) -> Result<(), Error> {
        let mut scans = self.active_scans.lock().await;
        if let Some(state) = scans.get_mut(&scan_id.to_string()) {
            let status_str: String = status.clone().into();
            let mut conn = self.pool.get().map_err(|e| Error::Database(e.to_string()))?;
            Scan::update_scan(&mut conn, scan_id, UpdateScan { status: status_str, result_path: None }) // TODO: REMOVE NAHUY
                .map_err(|e| Error::Database(e.to_string()))?;
            state.status = status;
        }
        Ok(())
    }

    async fn process_scan_result(
        &self,
        conn: &mut PgConnection,
        p_id: Uuid,
        scan_id: Uuid,
    ) -> Result<(), Error> {
        let output_file = self.scans_dir.join(scan_id.to_string()).join("result.xml");
        let output_file_str = output_file.to_str().ok_or(Error::IoError("Can't unwrap output file".to_string()))?;
        if !output_file.exists() {
            return Err(Error::ExecutionError(
                "Scan result file not found".to_string(),
            ));
        }

        let content =
            fs::read_to_string(&output_file).map_err(|e| Error::IoError(e.to_string()))?;

        let result_hosts = NmapService::parse_up_hosts(&content)?;
        let scan_result = NmapScanResult { output_file: output_file_str.to_string(), hosts: result_hosts };
        
        let mut scans = self.active_scans.lock().await;
        if let Some(state) = scans.get_mut(&scan_id.to_string()) {
            state.result = Some(scan_result);
            state.status = ScanStatus::Completed;
        }

        Ok(())
    }
}

#[async_trait]
impl VulnerabilityScanner for NmapService {
    type ScanRequest = NmapScanRequest;
    type ScanResult = NmapScanResult;

    async fn create_scan(
        &mut self,
        conn: &mut PgConnection,
        p_id: Uuid,
        request: Self::ScanRequest,
    ) -> Result<String, Error> {
        let scan_id = Uuid::new_v4();
        let output_dir = self.scans_dir.join(scan_id.to_string());
        fs::create_dir_all(&output_dir).map_err(|e| Error::IoError(e.to_string()))?;

        let mut scans = self.active_scans.lock().await;
        scans.insert(
            scan_id.to_string(),
            ScanState {
                status: ScanStatus::Pending,
                result: None,
            },
        );
        drop(scans);

        let output_file = output_dir.join("result.xml");
        
        // Create scan record in database
        let new_scan = NewScan {
            project_id: p_id,
            scanner_type: "nmap".to_string(),
            status: "pending".to_string(),
            target: request.target.clone(),
            result_path: Some(output_file.to_str().unwrap_or("").to_string()),
        };

        Scan::create_scan(conn, new_scan).map_err(|e| Error::Database(e.to_string()))?;
        
        // Start scan in background
        let service = self.clone();
        let scan_id_clone = scan_id;
        let target = request.target.clone();

        let pool = self.pool.clone();  // Клонируем пул соединений
        
        tokio::spawn(async move {
            let mut conn = match pool.get() {
                Ok(conn) => conn,
                Err(e) => {
                    error!("Failed to get DB connection: {}", e);
                    return;
                }
            };
            if let Err(e) = service
                .update_scan_status(scan_id_clone, ScanStatus::Running)
                .await
            {
                error!("Failed to update scan status: {}", e);
                return;
            }

            debug!("Run nmap scan for target: {}", target);
            
            let mut command = AsyncCommand::new("nmap");
            command.arg("-oX").arg(&output_file).arg(&target);

            match command.output().await {
                Ok(output) => {
                    if !output.status.success() {
                        let error = String::from_utf8_lossy(&output.stderr).into_owned();
                        let _ = service
                            .update_scan_status(scan_id_clone, ScanStatus::Failed(error))
                            .await;
                        return;
                    }
                    if let Err(e) = service.process_scan_result(&mut conn, p_id, scan_id_clone).await {
                        let _ = service
                            .update_scan_status(
                                scan_id_clone,
                                ScanStatus::Failed(e.to_string()),
                            )
                            .await;
                    }
                },
                Err(e) => {
                    let _ = service
                        .update_scan_status(scan_id_clone, ScanStatus::Failed(e.to_string()))
                        .await;
                },
            }
        });

        Ok(scan_id.to_string())
    }

    async fn get_scan_result(&self, task_id: &str) -> Result<Self::ScanResult, Error> {
        let scans = self.active_scans.lock().await;

        if let Some(state) = scans.get(task_id) {
            match &state.status {
                ScanStatus::Completed => {
                    if let Some(result) = &state.result {
                        return Ok(result.clone());
                    }
                },
                ScanStatus::Failed(error) => {
                    return Err(Error::ExecutionError(error.clone()));
                },
                _ => {
                    return Ok(NmapScanResult { output_file: "".to_string(), hosts: Vec::new() });
                },
            }
        }

        Ok(NmapScanResult { output_file: "".to_string(), hosts: Vec::new() })
    }

    async fn start_scan(&mut self, scan_id: Uuid, target: &str) -> Result<AnyScanResult, AppError> {
        let scan_path = format!("{}/{}", self.scans_dir.display(), scan_id);
        tokio::fs::create_dir_all(&scan_path).await.map_err(|e| {
            error!("Failed to create scan directory: {}", e);
            AppError::InternalServerError
        })?;

        let output_file = format!("{}/scan.xml", scan_path);

        let status = tokio::process::Command::new("nmap")
            .arg("-oX")
            .arg(&output_file)
            .arg(target)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await
            .map_err(|e| {
                error!("Failed to execute nmap: {}", e);
                AppError::InternalServerError
            })?;

        if !status.success() {
            error!("Nmap scan failed with status: {}", status);
            return Err(AppError::InternalServerError);
        }

        let xml_output = tokio::fs::read_to_string(&output_file).await.map_err(|e| {
            error!("Failed to read result file: {}", e);
            AppError::InternalServerError
        })?;

        let new_hosts = NmapService::parse_up_hosts(&xml_output).map_err(|e| {
            error!("Failed to parse result file: {}", e);
            AppError::InternalServerError
        })?;

        Ok(AnyScanResult::Nmap(NmapScanResult {
            output_file,
            hosts: new_hosts,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_up_hosts() {
        let xml = fs::read_to_string("/app/scans/0e430a96-0fb9-4626-ae5b-318472b0f033/scan.xml").unwrap();

        let hosts = NmapService::parse_up_hosts(&xml).unwrap();
        assert_eq!(hosts.len(), 1);
        assert!(hosts.contains(&"192.168.1.254".to_string()));
    }
}
