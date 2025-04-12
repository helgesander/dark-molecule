use uuid::Uuid;
use crate::db::schema::issues;
use crate::db::schema::projects;
use diesel::prelude::*;
use log::debug;
use serde::{Deserialize, Serialize};
use crate::dtos::handlers::IssueForm;
use crate::models::project::Project;

#[derive(Queryable, Selectable, Serialize, Identifiable, Associations, PartialEq, Debug)]
#[diesel(table_name = issues)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Project, foreign_key = project_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Issue {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub mitigation: Option<String>,
    pub cvss: f64,
    pub project_id: Uuid
}

#[derive(Insertable, Deserialize, AsChangeset, Debug)]
#[diesel(table_name = issues)]
struct NewIssue {
    name: String,
    description: Option<String>,
    mitigation: Option<String>,
    cvss: Option<f64>,
    project_id: Uuid
}

impl Issue {
    pub fn get_issues_by_project_id(conn: &mut PgConnection, id_project: Uuid) -> QueryResult<Vec<Issue>> {
        projects::table
            .find(id_project)
            .select(Project::as_select())
            .first(conn)
            .optional()?
            .map(|project| {
                Issue::belonging_to(&project)
                    .select(Issue::as_select())
                    .load(conn)
            })
            .unwrap_or_else(|| Ok(Vec::new()))
    }

    pub fn create_issue(conn: &mut PgConnection, form: &IssueForm, id_project: Uuid) -> QueryResult<Issue> {
        debug!("Create issue with data {:?}", form);
        let new_issue = NewIssue {
            name: form.name.clone(),
            description: form.description.clone(),
            mitigation: form.mitigation.clone(),
            cvss: form.cvss,
            project_id: id_project,
        };
        diesel::insert_into(issues::table)
            .values(new_issue)
            .get_result::<Issue>(conn)
    }

    pub fn update_issue(conn: &mut PgConnection, form: &IssueForm, id_project: Uuid, issue_id: Uuid) -> QueryResult<usize> {
        use crate::db::schema::issues::dsl::*;
        let new_issue = NewIssue {
            name: form.name.clone(),
            description: form.description.clone(),
            mitigation: form.mitigation.clone(),
            cvss: form.cvss,
            project_id: id_project
        };
        diesel::update(issues.filter(id.eq(issue_id)))
            .set(&new_issue)
            .execute(conn)
    }

    pub fn delete_issue(conn: &mut PgConnection, issue_id: Uuid) -> QueryResult<usize> {
        use crate::db::schema::issues::dsl::*; // Импортируем DSL

        conn.transaction(|conn| {
            diesel::delete(issues.filter(id.eq(issue_id))) // Используем DSL-алиас
                .execute(conn)
        })
    }

    pub fn get_issue(conn: &mut PgConnection, issue_id: Uuid) -> QueryResult<Option<Issue>> {
        use crate::db::schema::issues::dsl::*;
        issues
            .filter(id.eq(issue_id))
            .select(Issue::as_select())
            .first(conn)
            .optional()
    }
}