use crate::models::team::Team;
use uuid::Uuid;
use diesel::prelude::*;
use chrono::NaiveDate;
use log::debug;
use serde::{Deserialize, Serialize};
use crate::db::schema::projects::dsl::projects;
use crate::db::schema::projects::id;
use crate::dtos::db::ProjectForm;

use crate::models::user::User;

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::db::schema::projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Team))]
#[diesel(primary_key(id))]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub folder: String,
    pub team_id: Uuid
}
impl Project {
    pub fn get_project_by_id(conn: &mut PgConnection, project_id: Uuid) -> QueryResult<Option<Project>>{
        projects
            .filter(id.eq(project_id))
            .select(Project::as_select())
            .first(conn)
            .optional()
    }

    pub fn get_project_by_name(conn: &mut PgConnection, name: String) {
        unimplemented!()
    }

    pub fn create_project(conn: &mut PgConnection, form: &ProjectForm) -> QueryResult<Project> {
        debug!("Creating project with data {:?}", form);
        diesel::insert_into(projects)
            .values(form)
            .get_result::<Project>(conn)
    }

    pub fn update_project(conn: &mut PgConnection, form: &ProjectForm, project_id: Uuid) -> QueryResult<usize> {
        debug!("Update project with data {:?}", form);
        diesel::update(projects.filter(id.eq(project_id)))
            .set(form)
            .execute(conn)
    }

    pub fn delete_project(conn: &mut PgConnection, project_id: Uuid) -> QueryResult<usize> {
        debug!("Deleting project with data {:?}", project_id);
        diesel::delete(projects.filter(id.eq(project_id))).execute(conn)
    }
}