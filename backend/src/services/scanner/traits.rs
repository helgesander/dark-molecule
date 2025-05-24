use async_trait::async_trait;
use diesel::PgConnection;
use uuid::Uuid;
use crate::db::Pool;
use crate::utils::errors::AppError;

use crate::services::scanner::types::{AnyScanResult, Error};

#[async_trait]
pub trait VulnerabilityScanner: Send + Sync {
    type ScanRequest;
    type ScanResult;
    async fn create_scan(
        &mut self,
        conn: &mut PgConnection,
        project_id: Uuid,
        request: Self::ScanRequest,
    ) -> Result<String, Error>;
    async fn get_scan_result(&self, task_id: &str) -> Result<Self::ScanResult, Error>;
    async fn start_scan(&mut self, scan_id: Uuid, target: &str) -> Result<AnyScanResult, AppError>;
}
