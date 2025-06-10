use async_trait::async_trait;
use uuid::Uuid;
use crate::utils::errors::AppError;

use crate::services::scanner::types::{AnyScanResult, Error};

#[async_trait]
pub trait VulnerabilityScanner: Send + Sync {
    type ScanRequest;
    type ScanResult;
    async fn get_scan_result(&self, task_id: &str) -> Result<Self::ScanResult, Error>;
    async fn start_scan(&mut self, scan_id: Uuid, target: &str) -> Result<AnyScanResult, AppError>;
}
