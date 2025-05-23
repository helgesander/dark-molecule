use diesel::PgConnection;
use uuid::Uuid;

use crate::models::project::ProjectFullResponse;
use crate::models::report_template::ReportTemplate;
use crate::services::report::types::{Error, Report};

pub trait ReportGenerator {
    fn generate(
        &self,
        project: &ProjectFullResponse,
        report_template: &ReportTemplate,
    ) -> Result<Report, Error>;
    fn save_report(
        &self,
        conn: &mut PgConnection,
        project_id: Uuid,
        filename: String,
        data: Vec<u8>,
        template_id: i32,
    ) -> Result<(), Error>;
}
