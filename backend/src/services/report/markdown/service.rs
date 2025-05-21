use std::path::{Path, PathBuf};
use handlebars::{Handlebars, Template};
use uuid::Uuid;
use chrono::{Utc, NaiveDate};
use crate::models::project::ProjectFullResponse;
use crate::models::report_template::ReportTemplate;
use crate::services::report::traits::ReportGenerator;
use crate::services::report::types::{Error, Report};
use std::fs;
use std::io::Write;
use diesel::PgConnection;
use log::error;
use crate::utils::config::CONFIG;
use crate::models::report;

pub struct MarkdownService;

impl MarkdownService {
    fn register_helpers(handlebars: &mut Handlebars) {
        handlebars.register_helper("formatDate", Box::new(|h: &handlebars::Helper, _: &Handlebars, _: &handlebars::Context, _: &mut handlebars::RenderContext, out: &mut dyn handlebars::Output| {
            let param = h.param(0).ok_or_else(|| handlebars::RenderError::new("Param not found for helper 'formatDate'"))?;
            
            if let Some(date) = param.value().as_str() {
                if let Ok(dt) = NaiveDate::parse_from_str(date, "%Y-%m-%d") {
                    out.write(&dt.format("%d.%m.%Y").to_string())?;
                }
            }
            Ok(())
        }));

        handlebars.register_helper("countIssuesBySeverity", Box::new(|h: &handlebars::Helper, _: &Handlebars, _: &handlebars::Context, _: &mut handlebars::RenderContext, out: &mut dyn handlebars::Output| {
            let issues = h.param(0).ok_or_else(|| handlebars::RenderError::new("Param not found for helper 'countIssuesBySeverity'"))?;
            let severity = h.param(1).ok_or_else(|| handlebars::RenderError::new("Param not found for helper 'countIssuesBySeverity'"))?;
            
            if let Some(issues_array) = issues.value().as_array() {
                let count = issues_array.iter()
                    .filter(|issue| issue.get("severity").and_then(|s| s.as_str()) == Some(severity.value().as_str().unwrap_or("")))
                    .count();
                out.write(&count.to_string())?;
            }
            Ok(())
        }));
    }
}

impl ReportGenerator for MarkdownService {
    fn generate(&self, project: &ProjectFullResponse, report_template: &ReportTemplate) -> Result<Report, Error> {
        let mut handlebars = Handlebars::new();
        Self::register_helpers(&mut handlebars);

        handlebars.register_template_file("report", report_template.file_path.clone()).map_err(|e| {
            error!("Failed to parse template: {}", e);
            Error::TemplateError(format!("Failed to parse template: {}", e))
        })?;

        // Подготавливаем данные для шаблона
        let data = serde_json::json!({
            "project": project,
            "now": Utc::now()
        });

        // Рендерим отчет
        let rendered = handlebars.render("report", &data)
            .map_err(|e| Error::TemplateError(format!("Failed to render template: {}", e)))?;

        let report = Report {
            content: rendered.into_bytes(),
            format: "markdown".to_string(),
            generated_at: Utc::now(),
        };


        Ok(report)
    }

    fn save_report(&self, conn: &mut PgConnection, project_id: Uuid, data: Vec<u8>, template_id: i32) -> Result<(), Error> {
        let reports_dir = Path::new(&CONFIG.reports_path);
        if !reports_dir.exists() {
            fs::create_dir_all(reports_dir)
                .map_err(|e| Error::FileError(format!("Failed to create reports directory: {}", e)))?;
        }

        let filename = format!("report_{}_{}.md", project_id, Utc::now().format("%Y%m%d_%H%M%S"));
        let file_path = reports_dir.join(filename.clone());

        let mut file = fs::File::create(file_path.clone())
            .map_err(|e| Error::FileError(format!("Failed to create report file: {}", e)))?;

        file.write_all(&data)
            .map_err(|e| Error::FileError(format!("Failed to write report: {}", e)))?;

        report::Report::create_report(conn, &project_id, filename, file_path.to_str().unwrap(), data, template_id)
            .map_err(|e| {
                error!("Can't create report in database: {}", e);
                Error::DatabaseError(format!("Failed to create report: {}", e))
            })?;

        Ok(())
    }
}