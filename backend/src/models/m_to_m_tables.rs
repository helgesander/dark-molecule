use diesel::prelude::*;
use uuid::Uuid;

use crate::db::schema::{issue_hosts, users_projects, users_teams};

#[derive(Debug, Queryable, Identifiable, Selectable)]
#[diesel(table_name = users_projects)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Project))]
#[diesel(primary_key(user_id, project_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserProject {
    pub user_id: Uuid,
    pub project_id: Uuid,
}

#[derive(Debug, Queryable, Identifiable, Selectable)]
#[diesel(table_name = users_teams)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Team))]
#[diesel(primary_key(user_id, team_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserTeam {
    pub user_id: Uuid,
    pub team_id: Uuid,
}

#[derive(Debug, Queryable, Identifiable, Selectable)]
#[diesel(table_name = issue_hosts)]
#[diesel(belongs_to(Issue))]
#[diesel(belongs_to(Host))]
#[diesel(primary_key(issue_id, host_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct IssueHost {
    pub issue_id: Uuid,
    pub host_id: i32,
}
