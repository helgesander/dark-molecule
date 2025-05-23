use async_trait::async_trait;
use diesel::PgConnection;
use uuid::Uuid;

use crate::services::scanner::types::Error;

#[async_trait]
pub trait VulnerabilityScanner {
    type ScanRequest;
    type ScanResult;
    async fn create_scan(
        &self,
        conn: &mut PgConnection,
        project_id: Uuid,
        request: Self::ScanRequest,
    ) -> Result<String, Error>;
    async fn get_scan_result(&self, task_id: &str) -> Result<Self::ScanResult, Error>;
}
