pub mod nmap;
pub mod nuclei;
pub mod traits;
pub mod types;

use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use crate::db::Pool;
use crate::utils::config::AppConfig;

pub use traits::VulnerabilityScanner;
use crate::services::scanner::nmap::NmapService;
use crate::services::scanner::nuclei::NucleiService;
use crate::services::scanner::types::AnyScanResult;
use crate::utils::errors::AppError;

pub struct ScannerService {
    pub nmap: Arc<Mutex<nmap::NmapService>>,
    pub nuclei: Arc<Mutex<nuclei::NucleiService>>,
}

pub enum Scanner {
    Nmap(Arc<Mutex<NmapService>>),
    Nuclei(Arc<Mutex<NucleiService>>),
}

impl Scanner {
    pub async fn start_scan(&self, scan_id: Uuid, target: &str) -> Result<AnyScanResult, AppError> {
        match self {
            Self::Nmap(scanner) => {
                let mut scanner = scanner.lock().await;
                scanner.start_scan(scan_id, target).await
            },
            Self::Nuclei(scanner) => {
                let mut scanner = scanner.lock().await;
                scanner.start_scan(scan_id, target).await
            }
        }
    }
}


impl ScannerService {
    pub fn new(config: &AppConfig, pool: Pool) -> Self {
        Self {
            nmap: Arc::new(Mutex::new(nmap::NmapService::new(
                config.scans_path.clone(),
                pool,
            ))),
            nuclei: Arc::new(Mutex::new(nuclei::NucleiService::new(
                config.scans_path.clone(),
            ))),
        }
    }

    pub async fn get_nuclei(&self) -> Arc<Mutex<nuclei::NucleiService>> {
        self.nuclei.clone()
    }

    pub async fn get_nmap(&self) -> Arc<Mutex<nmap::NmapService>> {
        self.nmap.clone()
    }
}
