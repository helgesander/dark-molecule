use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::schema::scans;
use crate::models::project::Project;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Associations, Selectable)]
#[diesel(belongs_to(Project))]
#[diesel(table_name = scans)]
pub struct Scan {
    pub id: Uuid,
    pub project_id: Uuid,
    pub scanner_type: String,
    pub status: String,
    pub result_path: Option<String>,
    pub target: String
}

#[derive(Debug, Insertable)]
#[diesel(table_name = scans)]
pub struct NewScan {
    pub project_id: Uuid,
    pub scanner_type: String,
    pub status: String,
    pub result_path: Option<String>,
    pub target: String
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = scans)]
pub struct UpdateScan {
    pub status: String,
    pub result_path: Option<String>,
}

impl Scan {
    pub fn find_by_id(conn: &mut PgConnection, scan_id: Uuid) -> QueryResult<Option<Scan>> {
        use crate::db::schema::scans::dsl::*;
        scans.find(scan_id).first(conn).optional()
    }

    pub fn find_by_project(conn: &mut PgConnection, prjct_id: Uuid) -> QueryResult<Vec<Scan>> {
        use crate::db::schema::scans::dsl::*;
        scans.filter(project_id.eq(prjct_id)).load::<Scan>(conn)
    }

    pub fn create_scan(conn: &mut PgConnection, scan: NewScan) -> QueryResult<Scan> {
        use crate::db::schema::scans::dsl::*;
        diesel::insert_into(scans).values(scan).get_result::<Scan>(conn)
    }

    pub fn update_scan(
        conn: &mut PgConnection,
        scan_id: Uuid,
        scan: UpdateScan,
    ) -> QueryResult<Scan> {
        use crate::db::schema::scans::dsl::*;
        diesel::update(scans.find(scan_id))
            .set(scan)
            .get_result::<Scan>(conn)
    }

    pub fn delete_scan(conn: &mut PgConnection, scan_id: Uuid) -> QueryResult<usize> {
        use crate::db::schema::scans::dsl::*;
        diesel::delete(scans.filter(id.eq(scan_id))).execute(conn)
    }
}
