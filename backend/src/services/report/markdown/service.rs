use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use chrono::{NaiveDate, Utc};
use diesel::PgConnection;
use handlebars::{Handlebars, Template};
use log::{debug, error};
use uuid::Uuid;

use crate::models::project::ProjectFullResponse;
use crate::models::report;
use crate::models::report_template::ReportTemplate;
use crate::services::report::traits::ReportGenerator;
use crate::services::report::types::{Error, Report};
use crate::utils::config::CONFIG;

pub struct MarkdownService;

impl MarkdownService {
    fn register_helpers(handlebars: &mut Handlebars) {
        handlebars.register_helper(
            "formatDate",
            Box::new(
                |h: &handlebars::Helper,
                 _: &Handlebars,
                 _: &handlebars::Context,
                 _: &mut handlebars::RenderContext,
                 out: &mut dyn handlebars::Output| {
                    let param = h.param(0).ok_or_else(|| {
                        handlebars::RenderError::new("Param not found for helper 'formatDate'")
                    })?;

                    if let Some(date) = param.value().as_str() {
                        if let Ok(dt) = NaiveDate::parse_from_str(date, "%Y-%m-%d") {
                            out.write(&dt.format("%d.%m.%Y").to_string())?;
                        }
                    }
                    Ok(())
                },
            ),
        );

        handlebars.register_helper(
            "severityFromCvss",
            Box::new(
                |h: &handlebars::Helper,
                 _: &Handlebars,
                 _: &handlebars::Context,
                 _: &mut handlebars::RenderContext,
                 out: &mut dyn handlebars::Output| {
                    // Получаем параметр CVSS score
                    let cvss_score =
                        h.param(0).and_then(|v| v.value().as_f64()).ok_or_else(|| {
                            handlebars::RenderError::new("CVSS score must be a number")
                        })?;

                    // Определяем severity по стандартной классификации CVSS
                    let severity = if cvss_score >= 9.0 {
                        "Critical"
                    } else if cvss_score >= 7.0 {
                        "High"
                    } else if cvss_score >= 4.0 {
                        "Medium"
                    } else if cvss_score > 0.0 {
                        "Low"
                    } else {
                        "None"
                    };

                    out.write(severity)?;
                    Ok(())
                },
            ),
        );

        handlebars.register_helper(
            "countBySeverity",
            Box::new(
                |h: &handlebars::Helper,
                 _: &Handlebars,
                 _: &handlebars::Context,
                 _: &mut handlebars::RenderContext,
                 out: &mut dyn handlebars::Output| {
                    let issues = h.param(0).and_then(|v| v.value().as_array()).ok_or(
                        handlebars::RenderError::new(
                            "Param not found for helper 'countBySeverity'",
                        ),
                    )?;
                    let severity = h
                        .param(1)
                        .and_then(|v| v.value().as_str())
                        .ok_or(handlebars::RenderError::new("CVSS score must be a number"))?;

                    let count = issues
                        .iter()
                        .filter(|issue| {
                            let cvss = issue.get("cvss").and_then(|v| v.as_f64()).unwrap_or(0.0);
                            match severity {
                                "critical" => cvss >= 9.0,
                                "high" => cvss >= 7.0,
                                "medium" => cvss >= 4.0,
                                "low" => cvss > 0.0,
                                _ => false,
                            }
                        })
                        .count();

                    out.write(&count.to_string())?;
                    Ok(())
                },
            ),
        );

        handlebars.register_helper(
            "array_length",
            Box::new(
                |h: &handlebars::Helper,
                 _: &Handlebars,
                 _: &handlebars::Context,
                 _: &mut handlebars::RenderContext,
                 out: &mut dyn handlebars::Output| {
                    let array = h
                        .param(0)
                        .and_then(|v| v.value().as_array())
                        .ok_or_else(|| handlebars::RenderError::new("Expected an array"))?;
                    out.write(&array.len().to_string())?;
                    Ok(())
                },
            ),
        );
    }
}

impl ReportGenerator for MarkdownService {
    fn generate(
        &self,
        project: &ProjectFullResponse,
        report_template: &ReportTemplate,
    ) -> Result<Report, Error> {
        let mut handlebars = Handlebars::new();
        Self::register_helpers(&mut handlebars);

        handlebars
            .register_template_file("report", report_template.file_path.clone())
            .map_err(|e| {
                error!("Failed to parse template: {}", e);
                Error::TemplateError(format!("Failed to parse template: {}", e))
            })?;

        // Подготавливаем данные для шаблона
        let data = serde_json::json!({
            "project": project,
            "now": Utc::now()
        });

        let filename = format!("report_{}.md", Utc::now().format("%Y%m%d_%H%M%S"));

        // Рендерим отчет
        let rendered = handlebars
            .render("report", &data)
            .map_err(|e| Error::TemplateError(format!("Failed to render template: {}", e)))?;

        let report = Report {
            filename,
            content: rendered.into_bytes(),
            format: "markdown".to_string(),
            generated_at: Utc::now(),
        };

        Ok(report)
    }

    fn save_report(
        &self,
        conn: &mut PgConnection,
        project_id: Uuid,
        filename: String,
        data: Vec<u8>,
        template_id: i32,
    ) -> Result<(), Error> {
        let reports_dir = Path::new(&CONFIG.reports_path);
        if !reports_dir.exists() {
            fs::create_dir_all(reports_dir).map_err(|e| {
                Error::FileError(format!("Failed to create reports directory: {}", e))
            })?;
        }

        debug!("Save file with filename: {}", filename.clone());
        let file_path = reports_dir.join(filename.clone());

        let mut file = fs::File::create(file_path.clone())
            .map_err(|e| Error::FileError(format!("Failed to create report file: {}", e)))?;

        file.write_all(&data)
            .map_err(|e| Error::FileError(format!("Failed to write report: {}", e)))?;

        report::Report::create_report(
            conn,
            &project_id,
            filename,
            file_path.to_str().unwrap(),
            data,
            template_id,
        )
        .map_err(|e| {
            error!("Can't create report in database: {}", e);
            Error::DatabaseError(format!("Failed to create report: {}", e))
        })?;

        Ok(())
    }
}
