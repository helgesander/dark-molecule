use crate::services::scanner::VulnerabilityScanner;
use std::path::Path;
use crate::services::scanner::types::Error;

pub struct NmapScanService;

pub struct NmapScanResult {}
pub struct NmapScanRequest {}
pub struct NmapError {}

impl VulnerabilityScanner for NmapScanService {
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
