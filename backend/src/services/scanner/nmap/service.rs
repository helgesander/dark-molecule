use std::path::Path;
use crate::services::report::tr;

pub struct NmapScanService;

pub struct NmapScanResult {}
pub struct NmapScanRequest {}
pub struct NmapError {}

impl VulnerabilityScanner for NmapScanService {
    type ScanRequest = NmapScanRequest;
    type ScanResult = NmapScanResult;
    type Error = NmapError;

    async fn create_scan(&self, request: Self::ScanRequest) -> Result<String, Self::Error> {
        todo!()
    }

    async fn get_scan_result(&self, task_id: &str) -> Result<Self::ScanResult, Self::Error> {
        todo!()
    }

    fn run_scan(&self, request: Self::ScanRequest, _: &Path) -> Result<Self::ScanResult, Self::Error> {
        todo!()
    }
}
