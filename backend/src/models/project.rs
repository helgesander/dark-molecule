use chrono::NaiveDate;
use diesel::associations::HasTable;
use diesel::prelude::*;
use log::debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dtos::handlers::ProjectForm;
use crate::models::host::{Host, HostResponse};
use crate::models::issue::Issue;

#[derive(Queryable, Selectable, Serialize, Identifiable, Deserialize, Debug)]
#[diesel(table_name = crate::db::schema::projects)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Team))]
#[diesel(primary_key(id))]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub scope: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub folder: String,
    pub team_id: Uuid,
}

#[derive(Insertable, Deserialize, Debug, AsChangeset)]
#[diesel(table_name = crate::db::schema::projects)]
struct NewProject {
    name: String,
    description: Option<String>,
    scope: Option<String>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    folder: String,
    team_id: Uuid,
}

#[derive(Serialize, Debug)]
pub struct ProjectFullResponse {
    id: Uuid,
    name: String,
    description: String,
    scope: String,
    start_date: NaiveDate,
    end_date: NaiveDate,
    folder: String,
    team_id: Uuid,
    issues: Vec<Issue>,
    hosts: Vec<HostResponse>,
}

#[derive(Serialize)]
pub struct ProjectOverviewResponse {
    pub id: Uuid,
    pub name: String,
    pub scope: Option<String>,
}

impl Project {
    pub fn get_project_by_id(
        conn: &mut PgConnection,
        project_id: Uuid,
    ) -> QueryResult<Option<ProjectFullResponse>> {
        use crate::db::schema::projects::dsl::*;
        let project = projects
            .filter(id.eq(project_id))
            .select(Project::as_select())
            .first(conn)
            .optional()?;
        match project {
            Some(project) => Ok(Some(project.to_full_response(conn)?)),
            None => Ok(None),
        }
    }

    pub fn get_project_by_name(conn: &mut PgConnection, name: String) {
        unimplemented!()
    }

    pub fn get_projects(conn: &mut PgConnection) -> QueryResult<Vec<ProjectOverviewResponse>> {
        use crate::db::schema::projects::dsl::*;
        let all_projects = projects::table().load::<Project>(conn)?;
        let mut result: Vec<ProjectOverviewResponse> = Vec::new();
        for project in all_projects {
            result.push(ProjectOverviewResponse {
                id: project.id,
                name: project.name.clone(),
                scope: project.scope,
            });
        }
        Ok(result)
    }

    pub fn create_project(conn: &mut PgConnection, form: &ProjectForm) -> QueryResult<Project> {
        use crate::db::schema::projects::dsl::*;
        debug!("Creating project with data {:?}", form);
        // TODO: add method for creation of NewProject
        let new_project = NewProject {
            name: form.name.clone(),
            description: form.description.clone(),
            scope: form.scope.clone(),
            start_date: form.start_date,
            end_date: form.end_date,
            folder: form.folder.clone(),
            team_id: form.team_id,
        };
        diesel::insert_into(projects)
            .values(new_project)
            .get_result::<Project>(conn) // TODO: change this method to add work
                                         // with team id
    }

    pub fn update_project(
        conn: &mut PgConnection,
        form: &ProjectForm,
        project_id: Uuid,
    ) -> QueryResult<usize> {
        use crate::db::schema::projects::dsl::*;
        let new_project = NewProject {
            name: form.name.clone(),
            description: form.description.clone(),
            scope: form.scope.clone(),
            start_date: form.start_date,
            end_date: form.end_date,
            folder: form.folder.clone(),
            team_id: form.team_id,
        };
        debug!("Update project with data {:?}", form);
        diesel::update(projects.filter(id.eq(project_id)))
            .set(&new_project)
            .execute(conn)
    }

    pub fn delete_project(conn: &mut PgConnection, project_id: Uuid) -> QueryResult<usize> {
        use crate::db::schema::projects::dsl::*;
        // TODO: add transaction
        debug!("Deleting project with data {:?}", project_id);
        diesel::delete(projects.filter(id.eq(project_id))).execute(conn)
    }

    pub fn to_full_response(&self, conn: &mut PgConnection) -> QueryResult<ProjectFullResponse> {
        Ok(ProjectFullResponse {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone().unwrap_or_else(|| String::new()), /* TODO: maybe change after to Option */
            scope: self.scope.clone().unwrap_or_else(|| String::new()),
            start_date: self.start_date,
            end_date: self.end_date,
            folder: self.folder.clone(),
            team_id: self.team_id,
            issues: Issue::get_issues_by_project_id(conn, self.id)?,
            hosts: Host::get_hosts_by_project_id(conn, self.id)?,
        })
    }
}
