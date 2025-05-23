use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;

use async_trait::async_trait;
use diesel::PgConnection;
use log::{debug, error, info};
use quick_xml::de::from_str;
use quick_xml::events::Event;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use tokio::process::Command as AsyncCommand;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::models::host::HostResponse;
use crate::models::scan::{NewScan, Scan, UpdateScan};
use crate::services::scanner::types::Error;
use crate::services::scanner::VulnerabilityScanner;

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
#[serde(rename_all = "kebab-case")]
struct NmapRun {
    host: Vec<Host>,
}

#[derive(Debug, Deserialize)]
struct Host {
    status: Status,
    #[serde(rename = "address", default)]
    addresses: Vec<Address>,
}

#[derive(Debug, Deserialize)]
struct Status {
    #[serde(rename = "@state")]
    state: String,
}

#[derive(Debug, Deserialize)]
struct Address {
    #[serde(rename = "@addrtype")]
    addr_type: String,
    #[serde(rename = "@addr")]
    addr: String,
}

#[derive(Clone)]
pub struct NmapService {
    scans_dir: PathBuf,
    active_scans: Arc<Mutex<HashMap<String, ScanState>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NmapScanRequest {
    pub target: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NmapScanResult {
    pub hosts: Vec<HostResponse>,
}

impl NmapService {
    pub fn new(scans_dir: impl AsRef<Path>) -> Self {
        Self {
            scans_dir: scans_dir.as_ref().to_path_buf(),
            active_scans: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn parse_up_hosts(xml_content: &str) -> Result<Vec<String>, Error> {
        let nmap_run: NmapRun = from_str(xml_content)?;

        let up_hosts = nmap_run
            .host
            .into_iter()
            .filter(|h| h.status.state == "up")
            .flat_map(|h| h.addresses)
            .filter(|a| a.addr_type == "ipv4" || a.addr_type == "ipv6")
            .map(|a| a.addr)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        Ok(up_hosts)
    }

    async fn update_scan_status(
        &self,
        conn: &mut PgConnection,
        scan_id: Uuid,
        status: ScanStatus,
    ) -> Result<(), Error> {
        let mut scans = self.active_scans.lock().await;
        if let Some(state) = scans.get_mut(&scan_id.to_string()) {
            let status_str: String = status.clone().into();
            Scan::update_scan(conn, scan_id, UpdateScan { status: status_str })
                .map_err(|e| Error::Database(e.to_string()))?;
            state.status = status;
        }
        Ok(())
    }

    async fn process_scan_result(
        &self,
        conn: &mut PgConnection,
        scan_id: Uuid,
    ) -> Result<(), Error> {
        let output_file = self.scans_dir.join(scan_id.to_string()).join("result.xml");
        if !output_file.exists() {
            return Err(Error::ExecutionError(
                "Scan result file not found".to_string(),
            ));
        }

        let content =
            fs::read_to_string(&output_file).map_err(|e| Error::IoError(e.to_string()))?;

        let result_host = NmapService::parse_up_hosts(&content)?;
        let mut result: Vec<HostResponse> = Vec::new();

        for host in result_host {
            result.push(HostResponse {
                hostname: None,
                ip_address: host,
            });
        }

        let scan_result = NmapScanResult { hosts: result };

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
        &self,
        conn: &mut PgConnection,
        project_id: Uuid,
        request: Self::ScanRequest,
    ) -> Result<String, Error> {
        let scan_id = Uuid::new_v4();
        let output_dir = self.scans_dir.join(scan_id.to_string());
        fs::create_dir_all(&output_dir).map_err(|e| Error::IoError(e.to_string()))?;

        // Initialize scan state
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
            project_id,
            scanner_type: "nmap".to_string(),
            status: "pending".to_string(),
            target: request.target.clone(),
            result_path: Some(output_file.to_str().unwrap_or("").to_string()),
        };

        let scan = Scan::create_scan(conn, new_scan).map_err(|e| Error::Database(e.to_string()))?;

        // Start scan in background
        let service = self.clone();
        let scan_id_clone = scan_id;

        tokio::spawn(async move {
            if let Err(e) = service
                .update_scan_status(conn, scan_id_clone, ScanStatus::Running)
                .await
            {
                error!("Failed to update scan status: {}", e);
                return;
            }

            debug!("Run nmap scan for target: {}", request.target);

            let mut command = AsyncCommand::new("nmap");
            command.arg("-oX").arg(&output_file).arg(&request.target);

            match command.output().await {
                Ok(output) => {
                    if !output.status.success() {
                        let error = String::from_utf8_lossy(&output.stderr).into_owned();
                        let _ = service
                            .update_scan_status(conn, scan_id_clone, ScanStatus::Failed(error))
                            .await;
                        return;
                    }

                    if let Err(e) = service.process_scan_result(conn, scan_id_clone).await {
                        let _ = service
                            .update_scan_status(
                                conn,
                                scan_id_clone,
                                ScanStatus::Failed(e.to_string()),
                            )
                            .await;
                    }
                },
                Err(e) => {
                    let _ = service
                        .update_scan_status(conn, scan_id_clone, ScanStatus::Failed(e.to_string()))
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
                    return Ok(NmapScanResult { hosts: Vec::new() });
                },
            }
        }

        Ok(NmapScanResult { hosts: Vec::new() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_up_hosts() {
        let xml = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <nmaprun>
            <host>
                <status state="up"/>
                <address addr="192.168.1.1" addrtype="ipv4"/>
            </host>
            <host>
                <status state="down"/>
                <address addr="192.168.1.2" addrtype="ipv4"/>
            </host>
            <host>
                <status state="up"/>
                <address addr="2001:db8::1" addrtype="ipv6"/>
            </host>
            <runstats>
                <hosts up="2" down="1"/>
            </runstats>
        </nmaprun>
        "#;

        let hosts = NmapService::parse_up_hosts(xml).unwrap();
        assert_eq!(hosts.len(), 2);
        assert!(hosts.contains(&"192.168.1.1".to_string()));
        assert!(hosts.contains(&"2001:db8::1".to_string()));
        assert!(!hosts.contains(&"192.168.1.2".to_string()));
    }
}
