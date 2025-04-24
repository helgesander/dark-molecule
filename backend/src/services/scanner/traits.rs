use async_trait::async_trait;
use std::error::Error;
use std::path::Path;

#[async_trait]
pub trait VulnerabilityScanner {
    type ScanRequest;
    type ScanResult;
    type Error: Error + Send + Sync + 'static;
    async fn create_scan(&self, request: Self::ScanRequest) -> Result<String, Self::Error>;
    async fn get_scan_result(&self, task_id: &str) -> Result<Self::ScanResult, Self::Error>;

    fn run_scan(&self, request: Self::ScanRequest, &Path) -> Result<Self::ScanResult, Self::Error>;
}