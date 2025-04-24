use std::path::Path;
use crate::services::report::types::{Error, Report};

pub trait ReportGenerator {

    fn generate() -> Result<Report, Error>;
    async fn load_template(&mut self, template_path: &Path);
}