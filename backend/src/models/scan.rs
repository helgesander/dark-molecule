use crate::db::schema::scans;
use crate::services::scanner::types::ScanResult;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Project))]
#[diesel(table_name = scans)]
pub struct Scan {
    pub id: i32, // TODO: maybe change to u32 
    pub project_id: Uuid,
    pub scanner_type: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub scan_config: serde_json::Value,
    pub results_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = scans)]
pub struct NewScan {
    pub project_id: Uuid,
    pub scanner_type: String,
    pub status: String,
    pub scan_config: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = scans)]
pub struct UpdateScan {
    pub status: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub results_path: Option<String>,
} 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResponse {
    pub id: i32,
    pub project_id: Uuid,
    pub scanner_type: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub result: Option<ScanResult>,
}

impl From<Scan> for ScanResponse {
    fn from(scan: Scan) -> Self {
        Self {
            id: scan.id,
            project_id: scan.project_id,
            scanner_type: scan.scanner_type,
            status: scan.status,
            created_at: scan.created_at,
            started_at: scan.started_at,
            completed_at: scan.completed_at,
            error_message: scan.error_message,
            result: None,
        }
    }
}

impl Scan {
    pub fn get_scan_by_id(conn: &mut PgConnection, scan_id: i32) -> QueryResult<Option<Scan>> {
        scans.filter(id.eq(scan_id)).first(conn).optional()
    }

    pub fn get_scans_by_project_id(conn: &mut PgConnection, project_id: Uuid) -> QueryResult<Vec<Scan>> {
        scans.filter(project_id.eq(project_id)).load::<Scan>(conn)
    }

    pub fn create_scan(conn: &mut PgConnection, scan: NewScan) -> QueryResult<Scan> {
        diesel::insert_into(scans).values(scan).execute(conn)?;
    }

    pub fn update_scan(conn: &mut PgConnection, scan_id: i32, scan: UpdateScan) -> QueryResult<Scan> {
        diesel::update(scans.filter(id.eq(scan_id))).set(scan).execute(conn)?;
    }

    pub fn delete_scan(conn: &mut PgConnection, scan_id: i32) -> QueryResult<usize> {
        diesel::delete(scans.filter(id.eq(scan_id))).execute(conn)
    }
}