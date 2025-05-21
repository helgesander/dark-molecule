use crate::db::schema::issues;
use crate::db::schema::projects;
use crate::dtos::handlers::{IssueForm, CreateIssueForm};
use crate::models::host::{Host, HostResponse};
use crate::models::project::Project;
use crate::models::proof_of_concept::ProofOfConcept;
use diesel::prelude::*;
use log::debug;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::schema::*;

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
    pub project_id: Uuid, // TODO: remove
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
    hosts: Vec<HostResponse>,
    // pocs: Vec<ProofOfConcept>,
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
        form: &CreateIssueForm,
        id_project: Uuid,
    ) -> QueryResult<Issue> {
        debug!("Create issue with data {:?}", form);
        let new_issue = NewIssue {
            name: form.name.clone(),
            description: None,
            mitigation: None,
            cvss: 0.0,
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
        conn.transaction(|conn| {
            // Update the issue information
            let updated = diesel::update(issues::table)
                .filter(issues::id.eq(issue_id))
                .filter(issues::project_id.eq(id_project))
                .set((
                    issues::name.eq(&form.name),
                    issues::description.eq(&form.description),
                    issues::mitigation.eq(&form.mitigation),
                    issues::cvss.eq(form.cvss.unwrap_or(0.0)),
                ))
                .execute(conn)?;

            // First, delete all existing issue-host relationships
            diesel::delete(
                issue_hosts::table
                    .filter(issue_hosts::issue_id.eq(issue_id))
            ).execute(conn)?;

            // Then, create new relationships for all hosts in the form
            if !form.hosts.is_empty() {
                // Получаем ID хостов по одному, так как нам нужно учитывать опциональность hostname
                let mut host_ids = Vec::new();
                
                for host in &form.hosts {
                    let mut query = hosts::table.into_boxed();
                    
                    // Добавляем фильтр по IP-адресу
                    query = query.filter(hosts::ip_address.eq(&host.ip_address));
                    
                    // Если есть hostname, добавляем фильтр по нему
                    if let Some(hostname) = &host.hostname {
                        query = query.filter(hosts::hostname.eq(hostname));
                    }
                    
                    if let Ok(host_id) = query.select(hosts::id).first::<i32>(conn) {
                        host_ids.push(host_id);
                    }
                }

                if !host_ids.is_empty() {
                    let new_relations: Vec<(Uuid, i32)> = host_ids.iter()
                        .map(|&host_id| (issue_id, host_id))
                        .collect();

                    diesel::insert_into(issue_hosts::table)
                        .values(&new_relations.iter().map(|&(issue_id, host_id)| {
                            (
                                issue_hosts::issue_id.eq(issue_id),
                                issue_hosts::host_id.eq(host_id)
                            )
                        }).collect::<Vec<_>>())
                        .execute(conn)?;
                }
            }

            Ok(updated)
        })
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
        use crate::db::schema::issue_hosts::dsl::*;
        use crate::db::schema::hosts::dsl::*;

        // Сначала получаем ID хостов, связанных с уязвимостью
        let host_ids = issue_hosts
            .filter(issue_id.eq(self.id))
            .select(host_id)
            .load::<i32>(conn)?;

        // Затем получаем информацию о хостах
        let related_hosts = hosts
            .filter(id.eq_any(host_ids))
            .select((
                hostname,
                ip_address,
            ))
            .load::<(Option<String>, String)>(conn)?
            .into_iter()
            .map(|(host, ip)| HostResponse {
                hostname: host,
                ip_address: ip,
            })
            .collect();

        Ok(IssueFullResponse {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            mitigation: self.mitigation.clone(),
            cvss: self.cvss.clone(),
            hosts: related_hosts,
        })
    }
}
