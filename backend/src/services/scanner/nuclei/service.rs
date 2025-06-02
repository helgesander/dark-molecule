extern crate chrono;

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use async_trait::async_trait;
use log::error;
use serde::{Deserialize, Serialize};
use serde_json;
use uuid::Uuid;
use crate::utils::errors::AppError;
use crate::dtos::handlers::{HostForm, IssueForm};
use crate::services::scanner::types::{AnyScanResult, Error};
use crate::services::scanner::VulnerabilityScanner;

#[derive(Clone)]
pub struct NucleiService {
    scans_dir: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NucleiScanRequest {
    pub target: String,
    pub templates: Option<Vec<String>>,
    pub severity: Option<Vec<String>>,
    pub output_format: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NucleiScanResult {
    pub output_file: String,
    pub findings: Vec<IssueForm>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NucleiFinding {
    pub host: String,
    pub severity: String,
    pub info: NucleiFindingInfo,
    #[serde(rename = "matched-at")]
    pub matched_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NucleiFindingInfo {
    pub name: String,
    pub description: Option<String>,
    pub remediation: Option<String>,
}

impl NucleiService {
    pub fn new(scans_dir: impl AsRef<Path>) -> Self {
        Self {
            scans_dir: scans_dir.as_ref().to_path_buf(),
        }
    }

    fn severity_to_cvss(severity: &str) -> f64 {
        match severity.to_lowercase().as_str() {
            "critical" => 9.0,
            "high" => 7.0,
            "medium" => 5.0,
            "low" => 3.0,
            "info" => 1.0,
            _ => 3.0,
        }
    }

    pub fn parse_nuclei_output(content: &str) -> Result<Vec<NucleiFinding>, Error> {
        let raw_findings: Vec<serde_json::Value> = serde_json::from_str(content)
            .map_err(|e| Error::ParseError(e.to_string()))?;

        let mut findings = Vec::new();

        for raw_finding in raw_findings {
            let info = &raw_finding["info"];
            let template_name = info["name"]
                .as_str()
                .ok_or(Error::ParseError("Missing template name".to_string()))?
                .to_string();

            let host = raw_finding["host"]
                .as_str()
                .ok_or(Error::ParseError("Missing host".to_string()))?
                .to_string();

            let severity = info["severity"]
                .as_str()
                .ok_or(Error::ParseError("Missing severity".to_string()))?
                .to_string();

            let description = info["description"]
                .as_str()
                .map(|s| s.to_string());

            let remediation = info["remediation"]
                .as_str()
                .map(|s| s.to_string());

            let matched_at = raw_finding["matched-at"]
                .as_str()
                .ok_or(Error::ParseError("Missing matched-at".to_string()))?
                .to_string();

            findings.push(NucleiFinding {
                host,
                severity,
                matched_at,
                info: NucleiFindingInfo {
                    name: template_name,
                    description,
                    remediation,
                },
            });
        }

        Ok(findings)
    }

    pub fn parse_to_issues(
        findings: Vec<NucleiFinding>,
    ) -> Vec<IssueForm> {
        // Group by vulnerability name
        let mut grouped: HashMap<String, Vec<NucleiFinding>> = HashMap::new();

        for finding in findings {
            grouped.entry(finding.info.name.clone())
                .or_default()
                .push(finding);
        }

        // Convert to IssueForm
        grouped.into_iter().map(|(name, group)| {
            let description = if group.len() > 1 {
                format!(
                    "Found {} instances of this vulnerability.\nFirst occurrence: {}",
                    group.len(),
                    group[0].matched_at
                )
            } else {
                format!("Found at: {}", group[0].matched_at)
            };

            let cvss = Self::severity_to_cvss(&group[0].severity);
            let hosts = group.iter().map(|f| HostForm {
                hostname: None,
                ip_address: f.host.clone(),
            }).collect();

            IssueForm {
                name,
                description: Some(description),
                mitigation: group[0].info.remediation.clone(),
                cvss: Some(cvss),
                hosts,
            }
        }).collect()
    }
}

#[async_trait]
impl VulnerabilityScanner for NucleiService {
    type ScanRequest = NucleiScanRequest;
    type ScanResult = NucleiScanResult;

    async fn get_scan_result(&self, task_id: &str) -> Result<Self::ScanResult, Error> {
        let output_file = self.scans_dir.join(task_id).join("results.json");
        let output_file_str = output_file.to_str()
            .ok_or(Error::IoError("Can't unwrap output file".to_string()))?;

        if !output_file.exists() {
            return Ok(NucleiScanResult {
                output_file: output_file_str.to_string(),
                findings: vec![],
            });
        }

        let content = fs::read_to_string(&output_file)
            .map_err(|_| Error::IoError("Can't read output file".to_string()))?;

        let findings = Self::parse_nuclei_output(&content)?;
        let issues = Self::parse_to_issues(findings);

        Ok(NucleiScanResult {
            output_file: output_file_str.to_string(),
            findings: issues,
        })
    }

    async fn start_scan(&mut self, scan_id: Uuid, target: &str) -> Result<AnyScanResult, AppError> {
        let scan_path = format!("{}/{}", self.scans_dir.display(), scan_id);
        fs::create_dir_all(&scan_path).map_err(|e| {
            error!("Failed to create scan directory: {}", e);
            AppError::InternalServerError
        })?;

        let output_file = format!("{}/scan.json", scan_path);
        let status = tokio::process::Command::new("nuclei")
            .arg("-u")
            .arg(&target)
            .arg("-je")
            .arg(&output_file)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await
            .map_err(|e| {
                error!("Failed to start nuclei scan: {}", e);
                AppError::InternalServerError
            })?;

        if !status.success() {
            error!("Nuclei scanning failed with status: {}", status);
            return Err(AppError::InternalServerError);
        }

        let json_output = tokio::fs::read_to_string(&output_file).await.map_err(|e| {
            error!("Failed to read output file: {}", e);
            AppError::InternalServerError
        })?;

        let findings = NucleiService::parse_nuclei_output(&json_output)
            .map_err(|e| {
                error!("Can't parse nuclei output file: {}", e);
                AppError::InternalServerError
            })?;
        let findings = NucleiService::parse_to_issues(findings);

        Ok(AnyScanResult::Nuclei(NucleiScanResult {
            output_file,
            findings,
        }))
    }
}