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

    fn get_format_by_extension<'a>(&self, extension: String) -> String {
        match extension.as_str() {
            "docx" => "Microsoft Word".to_string(),
            "md" => "Markdown".to_string(),
            "html" => "HTML".to_string(),
            _ => "unknown".to_string(),
        }
    }
}
