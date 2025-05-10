use crate::services::scanner::VulnerabilityScanner;
use std::path::Path;
use std::path::PathBuf;
use crate::services::scanner::types::Error;

pub struct NmapService {
    scans_dir: PathBuf,
}

pub struct NmapScanResult {}
pub struct NmapScanRequest {}
pub struct NmapError {}

impl NmapService {
    pub fn new(scans_dir: impl AsRef<Path>) -> Self {
        Self {
            scans_dir: scans_dir.as_ref().to_path_buf(),
        }
    }
}

impl VulnerabilityScanner for NmapService {
    type ScanRequest = NmapScanRequest;
    type ScanResult = NmapScanResult;

    async fn create_scan(&self, request: Self::ScanRequest) -> Result<String, Error> {
        todo!()
    }

    async fn get_scan_result(&self, task_id: &str) -> Result<Self::ScanResult, Error> {
        todo!()
    }

    fn run_scan(
        &self,
        request: Self::ScanRequest,
        _: &Path,
    ) -> Result<Self::ScanResult, Error> {
        todo!()
    }
}
