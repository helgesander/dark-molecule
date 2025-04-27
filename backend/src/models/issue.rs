use crate::db::schema::issues;
use crate::db::schema::projects;
use crate::dtos::handlers::IssueForm;
use crate::models::host::Host;
use crate::models::project::Project;
use crate::models::proof_of_concept::ProofOfConcept;
use diesel::prelude::*;
use log::debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    pub project_id: Uuid,
}

#[derive(Insertable, Deserialize, AsChangeset, Debug)]
#[diesel(table_name = issues)]
struct NewIssue {
    name: String,
    description: Option<String>,
    mitigation: Option<String>,
    cvss: f64,
    project_id: Uuid,
}

#[derive(Serialize, Debug)]
pub struct IssueFullResponse {
    id: Uuid,
    name: String,
    description: Option<String>,
    mitigation: Option<String>,
    cvss: f64,
    hosts: Vec<Host>,
    pocs: Vec<ProofOfConcept>,
}

impl Issue {
    pub fn get_issues_by_project_id(
        conn: &mut PgConnection,
        id_project: Uuid,
    ) -> QueryResult<Vec<Issue>> {
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

    pub fn create_issue(
        conn: &mut PgConnection,
        form: &IssueForm,
        id_project: Uuid,
    ) -> QueryResult<Issue> {
        debug!("Create issue with data {:?}", form);
        let new_issue = NewIssue {
            name: form.name.clone(),
            description: form.description.clone(),
            mitigation: form.mitigation.clone(),
            cvss: form.cvss.unwrap_or_else(|| 0.0),
            project_id: id_project,
        };
        diesel::insert_into(issues::table)
            .values(new_issue)
            .get_result::<Issue>(conn)
    }

    pub fn update_issue(
        conn: &mut PgConnection,
        form: &IssueForm,
        id_project: Uuid,
        issue_id: Uuid,
    ) -> QueryResult<usize> {
        use crate::db::schema::issues::dsl::*;
        let new_issue = NewIssue {
            name: form.name.clone(),
            description: form.description.clone(),
            mitigation: form.mitigation.clone(),
            cvss: form.cvss.unwrap_or_else(|| 0.0),
            project_id: id_project,
        };
        diesel::update(issues.filter(id.eq(issue_id)))
            .set(&new_issue)
            .execute(conn)
    }

    pub fn delete_issue(conn: &mut PgConnection, issue_id: Uuid) -> QueryResult<usize> {
        use crate::db::schema::issues::dsl::*;

        conn.transaction(|conn| diesel::delete(issues.filter(id.eq(issue_id))).execute(conn))
    }

    pub fn get_issue(
        conn: &mut PgConnection,
        issue_id: Uuid,
    ) -> QueryResult<Option<IssueFullResponse>> {
        use crate::db::schema::issues::dsl::*;
        let issue = issues
            .filter(id.eq(issue_id))
            .select(Issue::as_select())
            .first(conn)
            .optional()?;
        match issue {
            Some(issue) => Ok(Some(issue.to_full_response(conn)?)),
            None => Ok(None),
        }
    }

    fn to_full_response(&self, conn: &mut PgConnection) -> QueryResult<IssueFullResponse> {
        Ok(IssueFullResponse {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            mitigation: self.mitigation.clone(),
            cvss: self.cvss.clone(),
            hosts: Host::get_hosts_by_project_id(conn, self.project_id)?,
            pocs: ProofOfConcept::get_pocs_by_issue_id(conn, self.id)?,
        })
    }
}
