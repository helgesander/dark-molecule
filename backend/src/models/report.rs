use std::fs;
use chrono::Utc;
use diesel::prelude::*;
use log::{debug, error};
use crate::models::report_template::ReportTemplate;
use crate::db::schema;
use uuid::Uuid;
use crate::db::schema::projects;
use serde::{Serialize, Deserialize};
use crate::db::schema::reports::dsl::*;
use crate::models::project::Project;
use crate::utils::config::CONFIG;

#[derive(Queryable, Selectable, Serialize, Identifiable, Associations, PartialEq, Debug)]
#[diesel(table_name = schema::reports)]
#[diesel(belongs_to(ReportTemplate, foreign_key = template_id))]
#[diesel(belongs_to(Project, foreign_key = project_id))]
pub struct Report {
    pub id: i32,
    pub name: String,
    pub file_path: String,
    pub template_id: i32,
    pub project_id: Uuid
}

#[derive(Insertable)]
#[diesel(table_name = schema::reports)]
pub struct NewReport {
    pub name: String,
    pub file_path: String,
    pub template_id: i32,
    pub project_id: Uuid
}

#[derive(Serialize, Deserialize)]
pub struct ReportPreview {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReportData {
    pub filename: String,
    pub data: Vec<u8>
}

impl Report {
    pub fn get_reports_preview_by_project_id(conn: &mut PgConnection, prjct_id: Uuid) -> QueryResult<Vec<ReportPreview>> {
        use crate::db::schema::reports::dsl::*;
        let project_for_reports = projects::table
            .find(prjct_id)
            .select(Project::as_select())
            .first(conn)
            .optional()?;

        if let Some(project) = project_for_reports {
            let reports_tuple_vec = Report::belonging_to(&project)
                .select((id, name))
                .load::<(i32, String)>(conn)?;

            let mut reports_vec: Vec<ReportPreview> = Vec::new();
            for report_tuple in reports_tuple_vec {
                let report = ReportPreview {
                    id: report_tuple.0,
                    name: report_tuple.1
                };
                reports_vec.push(report)
            }

            Ok(reports_vec)
        } else {
            Err(diesel::NotFound)
        }
    }

    pub fn create_report(conn: &mut PgConnection, proj_id: &Uuid, filename: String, filepath: &str, report_data: Vec<u8>, templ_id: i32) -> QueryResult<ReportData> {
        conn.transaction(|conn| {
            let new_report = NewReport {
                name: filename.clone(),
                file_path: filepath.to_string(),
                template_id: templ_id,
                project_id: *proj_id
            };

            diesel::insert_into(reports)
                .values(&new_report)
                .get_result::<Report>(conn)?;

            Ok(ReportData {
                filename,
                data: report_data
            })
        })
    }

    pub fn get_report_data(conn: &mut PgConnection, report_id: i32) -> QueryResult<ReportData> {
        use crate::db::schema::reports::dsl::*;
        let (report_name, report_path) = reports
            .filter(id.eq(report_id))
            .select((name, file_path))
            .first::<(String, String)>(conn)?;

        debug!("Read file with name: {}", report_name.clone());

        let report_data = fs::read(&report_path).map_err(|e| {
            error!("Unable to read report file: {}", e);
            diesel::result::Error::DeserializationError(Box::new(e))
        })?;

        Ok(ReportData {
            filename: report_name,
            data: report_data
        })
    }
}