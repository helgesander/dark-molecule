use uuid::Uuid;
use diesel::prelude::*;
use crate::db::schema::{users_teams, users_projects};
use crate::models::{
    user::User,
    team::Team,
    project::Project
};

#[derive(Debug, Queryable, Identifiable, Selectable)]
#[diesel(table_name = users_projects)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Project))]
#[diesel(primary_key(user_id, project_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserProject {
    pub user_id: Uuid,
    pub project_id: Uuid
}

#[derive(Debug, Queryable, Identifiable, Selectable)]
#[diesel(table_name = users_teams)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Team))]
#[diesel(primary_key(user_id, team_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserTeam {
    pub user_id: Uuid,
    pub team_id: Uuid
}
