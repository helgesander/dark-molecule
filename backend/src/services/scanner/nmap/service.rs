use crate::services::scanner::VulnerabilityScanner;
use std::path::{Path, PathBuf};
use crate::services::scanner::types::Error;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use async_trait::async_trait;
use std::process::{Command, Stdio};
use std::fs;
use quick_xml::de::from_str;
use log::{error, info};

pub struct NmapService {
    scans_dir: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NmapScanRequest {
    pub target: String,
    pub ports: Option<String>,
    pub options: Option<String>,
    pub os_detection: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NmapScanResult {
    pub hosts: Vec<Host>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Host {
    pub address: String,
    pub hostnames: Vec<Hostname>,
    pub ports: Vec<Port>,
    pub os: Option<OsInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hostname {
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Port {
    pub protocol: String,
    pub portid: u16,
    pub state: String,
    pub service: Option<Service>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub product: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsInfo {
    pub name: String,
    pub accuracy: u8,
    pub osclass: Vec<OsClass>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsClass {
    pub vendor: String,
    pub osgen: Option<String>,
    pub r#type: String,
    pub accuracy: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsMatch {
    pub name: String,
    pub accuracy: u8,
    pub osclass: Vec<OsClass>,
}

impl NmapService {
    pub fn new(scans_dir: impl AsRef<Path>) -> Self {
        Self {
            scans_dir: scans_dir.as_ref().to_path_buf(),
        }
    }
}

#[async_trait]
impl VulnerabilityScanner for NmapService {
    type ScanRequest = NmapScanRequest;
    type ScanResult = NmapScanResult;

    async fn create_scan(&self, request: Self::ScanRequest) -> Result<String, Error> {
        let task_id = Uuid::new_v4().to_string();
        let output_dir = self.scans_dir.join(&task_id);
        fs::create_dir_all(&output_dir).map_err(|e| Error::IoError(e))?;
        
        let output_file = output_dir.join("result.xml");
        self.run_scan(request, &output_file)?;
        
        Ok(task_id)
    }

    async fn get_scan_result(&self, task_id: &str) -> Result<Self::ScanResult, Error> {
        let output_file = self.scans_dir.join(task_id).join("result.xml");
        if !output_file.exists() {
            return Ok(NmapScanResult {
                hosts: Vec::new(),
            });
        }
        
        let content = fs::read_to_string(&output_file)
            .map_err(|e| Error::IoError(e))?;
            
        let result: NmapScanResult = from_str(&content)
            .map_err(|e| Error::ParseError(format!("Failed to parse XML: {}", e)))?;
            
        Ok(result)
    }

    fn run_scan(
        &self,
        request: Self::ScanRequest,
        output_file: &Path,
    ) -> Result<Self::ScanResult, Error> {
        let mut command = Command::new("nmap");
        
        // Базовые параметры
        command
            .arg("-oX")
            .arg(output_file)
            .arg("-sV") // Версии сервисов
            .arg("-O")  // Определение ОС
            .arg("-F")  // Быстрое сканирование (только популярные порты)
            .arg(&request.target);
            
        // Добавляем дополнительные опции, если указаны
        if let Some(ports) = &request.ports {
            command.arg("-p").arg(ports);
        }
        
        if request.os_detection {
            command.arg("-O");
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
        
        let xml_content = std::fs::read_to_string(output_file)
            .map_err(|e| Error::IoError(e))?;
            
        let result: NmapScanResult = from_str(&xml_content)
            .map_err(|e| Error::ParseError(format!("Failed to parse XML: {}", e)))?;
            
        Ok(result)
    }
}

// Структуры для парсинга XML
#[derive(Debug, Deserialize)]
struct NmapRun {
    #[serde(rename = "host")]
    hosts: Vec<HostElement>,
}

#[derive(Debug, Deserialize)]
struct HostElement {
    address: Address,
    hostnames: Option<Hostnames>,
    ports: Option<Ports>,
    os: Option<Os>,
}

#[derive(Debug, Deserialize)]
struct Address {
    #[serde(rename = "addr")]
    addr: String,
}

#[derive(Debug, Deserialize)]
struct Hostnames {
    #[serde(rename = "hostname")]
    hostname: Vec<Hostname>,
}

#[derive(Debug, Deserialize)]
struct Ports {
    #[serde(rename = "port")]
    port: Option<Vec<PortElement>>,
}

#[derive(Debug, Deserialize)]
struct PortElement {
    #[serde(rename = "portid")]
    portid: u16,
    state: State,
    service: Option<ServiceElement>,
}

#[derive(Debug, Deserialize)]
struct State {
    #[serde(rename = "state")]
    state: String,
}

#[derive(Debug, Deserialize)]
struct ServiceElement {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "product")]
    product: Option<String>,
    #[serde(rename = "version")]
    version: Option<String>,
    #[serde(rename = "extrainfo")]
    extrainfo: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Os {
    #[serde(rename = "osmatch")]
    osmatch: Vec<OsMatch>,
}
