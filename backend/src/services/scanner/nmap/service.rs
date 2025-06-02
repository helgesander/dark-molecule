use std::collections::HashMap;

use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use async_trait::async_trait;
use log::error;
use quick_xml::de::from_str;
use quick_xml::DeError;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use uuid::Uuid;


use crate::services::scanner::types::{AnyScanResult, Error};
use crate::services::scanner::VulnerabilityScanner;
use crate::db::Pool;
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
}

#[async_trait]
impl VulnerabilityScanner for NmapService {
    type ScanRequest = NmapScanRequest;
    type ScanResult = NmapScanResult;

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
