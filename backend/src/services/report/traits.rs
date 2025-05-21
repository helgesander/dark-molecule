use diesel::PgConnection;
use crate::models::project::ProjectFullResponse;
use crate::models::report_template::ReportTemplate;
use crate::services::report::types::{Error, Report};
use uuid::Uuid;

pub trait ReportGenerator {
    fn generate(&self, project: &ProjectFullResponse, report_template: &ReportTemplate) -> Result<Report, Error>;
    fn save_report(&self, conn: &mut PgConnection, project_id: Uuid, data: Vec<u8>, template_id: i32) -> Result<(), Error>;
}
