use async_trait::async_trait;
use crate::services::scanner::types::Error;
use std::path::Path;

#[async_trait]
pub trait VulnerabilityScanner {
    type ScanRequest;
    type ScanResult;
    async fn create_scan(&self, request: Self::ScanRequest) -> Result<String, Error>;
    async fn get_scan_result(&self, task_id: &str) -> Result<Self::ScanResult, Error>;

    fn run_scan(
        &self,
        request: Self::ScanRequest,
        _: &Path,
    ) -> Result<Self::ScanResult, Error>;
}
