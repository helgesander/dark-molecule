use crate::services::report::types::{Error, Report};
use std::path::Path;

pub trait ReportGenerator {
    fn generate() -> Result<Report, Error>;
    async fn load_template(&mut self, template_path: &Path);
}
