use uuid::Uuid;
use crate::db::schema::issues;
use crate::models::project::Project;
use diesel::prelude::*;
#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = issues)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(project))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Issue {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub mitigation: String,
    pub cvss: f64,
    pub project_id: Uuid
}
