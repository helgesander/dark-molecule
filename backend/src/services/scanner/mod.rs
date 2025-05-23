pub mod nmap;
pub mod nuclei;
pub mod traits;
pub mod types;

use std::sync::Arc;

use tokio::sync::Mutex;
pub use traits::VulnerabilityScanner;
pub use types::ScanStatus;

use crate::utils::config::AppConfig;

#[derive(Clone)]
pub struct ScannerService {
    nuclei: Arc<Mutex<nuclei::NucleiService>>,
    nmap: Arc<Mutex<nmap::NmapService>>,
}

impl ScannerService {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            nuclei: Arc::new(Mutex::new(nuclei::NucleiService::new(
                config.scans_path.clone(),
            ))),
            nmap: Arc::new(Mutex::new(nmap::NmapService::new(
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
