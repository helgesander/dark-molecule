use std::fs;
use crate::db::schema::report_templates;
use diesel::prelude::*;
use diesel::{Identifiable, PgConnection, QueryResult, Queryable, RunQueryDsl, Selectable};
use serde::Serialize;
use crate::dtos::handlers::{ReportTemplateForm, UploadReportTemplateForm};
use log::error;
use crate::utils::errors::AppError;
use crate::utils::config::CONFIG;

#[derive(Queryable, Selectable, Identifiable, Serialize)]
#[diesel(table_name = report_templates)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
pub struct ReportTemplate {
    pub id: i32,
    pub name: String,
    pub extension: String,
    pub file_path: String,
}

#[derive(Insertable)]
#[diesel(table_name = report_templates)]
pub struct NewReportTemplate {
    pub name: String,
    pub extension: String,
    pub file_path: String,
}
#[derive(Serialize)]
pub struct ReportTemplatePreview {
    pub id: i32,
    pub name: String,
    pub extension: String,
}

impl ReportTemplate {
    pub fn get_template_by_id(conn: &mut PgConnection, template_id: i32) -> QueryResult<Option<ReportTemplate>> {
        use crate::db::schema::report_templates::dsl::*;
        
        report_templates
            .filter(id.eq(template_id))
            .select(ReportTemplate::as_select())
            .first(conn)
            .optional()
    }

    pub fn create_template(conn: &mut PgConnection, template: &ReportTemplateForm) -> QueryResult<()> {
        use crate::db::schema::report_templates::dsl::*;

        let template_file_path = format!("{}/{}", CONFIG.templates_path, template.filename);
        let file_extension = template.filename
            .split('.')
            .last()
            .unwrap_or("txt")
            .to_string();

        fs::write(&template_file_path, template.file.clone())
            .map_err(|e| {
                error!("Unable to write template file: {}", e);
                diesel::result::Error::DeserializationError(Box::new(e))
            })?;

        diesel::insert_into(report_templates)
            .values(NewReportTemplate {
                name: template.name.to_string(),
                extension: file_extension,
                file_path: template_file_path,
            })
            .execute(conn)
            .map(|_| ())
    }

    pub fn get_all_templates(conn: &mut PgConnection) -> QueryResult<Vec<ReportTemplatePreview>> {
        use crate::db::schema::report_templates::dsl::*;

        let templates = report_templates
            .select((id, name, extension))
            .load::<(i32, String, String)>(conn)?;

        Ok(templates
            .into_iter()
            .map(|(template_id, template_name, template_extension)| ReportTemplatePreview {
                id: template_id,
                name: template_name,
                extension: template_extension,
            })
            .collect())
    }
}
