use diesel::associations::HasTable;
use uuid::Uuid;
use crate::db::schema::issues;
use crate::db::schema::projects;
use diesel::prelude::*;
use log::debug;
use serde::Serialize;
use crate::db::schema::issues::dsl::*;
use crate::db::schema::projects::id;
use crate::dtos::db::IssueForm;
use crate::models::project::Project;

#[derive(Queryable, Selectable, Serialize, Identifiable, Associations, PartialEq, Debug)]
#[diesel(table_name = crate::db::schema::issues)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Project, foreign_key = project_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Issue {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub mitigation: String,
    pub cvss: f64,
    pub project_id: Uuid
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

    pub fn create_issue(conn: &mut PgConnection, form: &IssueForm) -> QueryResult<Issue> {
        debug!("Create issue with data {:?}", form);
        diesel::insert_into(issues::table)
            .values(form)
            .get_result::<Issue>(conn)
    }

    pub fn update_issue(conn: &mut PgConnection, form: &IssueForm, issue_id: Uuid) -> QueryResult<usize> {
        diesel::update(issues.filter(id.eq(issue_id)))
            .set(form)
            .execute(conn)
    }

    pub fn delete_issue(conn: &mut PgConnection, issue_id: Uuid) -> QueryResult<usize> {
        conn.transaction(|conn| {
            let count = diesel::delete(projects::table).filter(projects::id.eq(issue_id)).execute(conn)?;
            Ok(count)
        })
    }

    pub fn get_issue(conn: &mut PgConnection, issue_id: Uuid) -> QueryResult<Option<Issue>> {
        issues
            .filter(id.eq(issue_id))
            .select(Issue::as_select())
            .first(conn)
            .optional()
    }
}