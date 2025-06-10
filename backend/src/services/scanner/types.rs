use diesel::PgConnection;
use log::error;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;
use crate::dtos::handlers::HostForm;
use crate::models::host::Host;
use crate::models::issue::Issue;
use crate::services::scanner::nmap::service::NmapScanResult;
use crate::services::scanner::nuclei::service::NucleiScanResult;
use crate::utils::errors::AppError;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Vulnerability {
    pub name: String,
    pub severity: String,
    pub description: String,
}


#[derive(Serialize)]
pub enum AnyScanResult {
    Nmap(NmapScanResult),
    Nuclei(NucleiScanResult),
}

impl AnyScanResult {

    pub fn save_data(&self, project_id: Uuid, conn: &mut PgConnection) -> Result<(), AppError> {
        match &self {
            AnyScanResult::Nmap(res) => {
                let mut new_hosts_vec: Vec<HostForm> = Vec::new();
                for ip_address in res.hosts.clone() {
                    new_hosts_vec.push(HostForm {
                        hostname: None,
                        ip_address,
                    })
                };
                Host::create_hosts(conn, new_hosts_vec, project_id)
                    .map_err(|e| {
                        error!("Error creating hosts: {:?}", e);
                        AppError::DatabaseError
                    })?;
                Ok(())
            }
            AnyScanResult::Nuclei(res) => {
                Issue::create_issues(conn, res.findings.clone(), project_id)
                    .map_err(|e| {
                        error!("Error creating issues: {:?}", e);
                        AppError::DatabaseError
                    })?;
                Ok(())
            }
        }
    }
}
