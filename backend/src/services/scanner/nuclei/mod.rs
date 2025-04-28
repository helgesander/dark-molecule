pub mod service;

use crate::db::schema::scans;
use crate::models::scan::{NewScan, Scan, UpdateScan};
use crate::services::scanner::traits::VulnerabilityScanner;
use crate::services::scanner::types::{Error, ScanResult, NucleiScanResult};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use serde_json::Value;
use std::path::Path;
use uuid::Uuid;

pub struct NucleiService {
    pool: Pool<ConnectionManager<PgConnection>>,
    config: AppConfig,
}

impl NucleiService {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>, config: AppConfig) -> Self {
        Self { pool, config }
    }

    pub async fn get_scan(&self, scan_id: Uuid) -> Result<Scan, Error> {
        let conn = &mut self.pool.get().map_err(|e| Error::Database(e.to_string()))?;
        
        scans::table
            .find(scan_id)
            .first(conn)
            .map_err(|e| Error::Database(e.to_string()))
    }

    pub async fn update_scan(&self, scan_id: Uuid, update: UpdateScan) -> Result<Scan, Error> {
        let conn = &mut self.pool.get().map_err(|e| Error::Database(e.to_string()))?;
        
        diesel::update(scans::table.find(scan_id))
            .set(update)
            .get_result(conn)
            .map_err(|e| Error::Database(e.to_string()))
    }
}

#[async_trait]
impl VulnerabilityScanner for NucleiService {
    type ScanRequest = Value;
    type ScanResult = NucleiScanResult;
    type ScanStatus = Scan;

    async fn create_scan(&self, request: Self::ScanRequest) -> Result<String, Error> {
        let conn = &mut self.pool.get().map_err(|e| Error::Database(e.to_string()))?;
        
        let new_scan = NewScan {
            project_id: request["project_id"].as_str()
                .and_then(|s| Uuid::parse_str(s).ok())
                .ok_or_else(|| Error::InvalidRequest("Invalid project_id".to_string()))?,
            scanner_type: "nuclei".to_string(),
            status: "pending".to_string(),
            scan_config: request,
        };

        let scan: Scan = diesel::insert_into(scans::table)
            .values(new_scan)
            .get_result(conn)
            .map_err(|e| Error::Database(e.to_string()))?;

        Ok(scan.id.to_string())
    }

    async fn get_scan_result(&self, task_id: &str) -> Result<Self::ScanResult, Error> {
        let scan_id = Uuid::parse_str(task_id)
            .map_err(|e| Error::InvalidRequest(e.to_string()))?;
            
        let scan = self.get_scan(scan_id).await?;
        
        // Здесь логика получения результатов сканирования Nuclei
        let result = NucleiScanResult {
            vulnerabilities: vec![], // Заполнить реальными данными
            scan_time: 0.0,          // Заполнить реальными данными
        };
        
        Ok(result)
    }

    async fn get_scan_status(&self, task_id: &str) -> Result<Self::ScanStatus, Error> {
        let scan_id = Uuid::parse_str(task_id)
            .map_err(|e| Error::InvalidRequest(e.to_string()))?;
            
        self.get_scan(scan_id).await
    }

    fn run_scan(
        &self,
        request: Self::ScanRequest,
        output_path: &Path,
    ) -> Result<Self::ScanResult, Error> {
        // Здесь логика запуска сканирования
        // ...
        
        Ok(serde_json::json!({}))
    }
}
