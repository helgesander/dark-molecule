pub mod nmap;
pub mod nuclei;
pub mod traits;
pub mod types;

use std::sync::Arc;
use tokio::sync::Mutex;
use crate::db::Pool;
use crate::utils::config::AppConfig;

pub use traits::VulnerabilityScanner;
pub use types::ScanStatus;

pub struct ScannerService {
    pub nmap: Arc<Mutex<nmap::NmapService>>,
    nuclei: Arc<Mutex<nuclei::NucleiService>>,
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
