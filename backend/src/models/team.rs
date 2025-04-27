use crate::db::schema::users::id;
use crate::{
    db::schema::teams, db::schema::users::dsl::users, dtos::db::TeamForm, models::user::User,
};
use diesel::prelude::*;
use log::debug;
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Identifiable, Associations, PartialEq, Debug)]
#[diesel(table_name = crate::db::schema::teams)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
#[diesel(belongs_to(User, foreign_key = admin_id))]
pub struct Team {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    admin_id: Uuid,
}

impl Team {
    pub fn create_team(conn: &mut PgConnection, form: &TeamForm) -> QueryResult<Team> {
        debug!("Create team with data: {:?}", form);
        diesel::insert_into(teams::table)
            .values(form)
            .get_result::<Team>(conn)
    }

    pub fn get_teams(conn: &mut PgConnection) -> QueryResult<Vec<Team>> {
        debug!("Get all teams");
        teams::table.load::<Team>(conn)
    }

    pub fn get_teams_by_admin_id(
        conn: &mut PgConnection,
        admin_id: Uuid,
    ) -> QueryResult<Vec<Team>> {
        debug!("Get teams by admin id {}", admin_id);
        users
            .filter(id.eq(admin_id))
            .select(User::as_select())
            .first(conn)
            .optional()?
            .map(|user| {
                Team::belonging_to(&user)
                    .select(Team::as_select())
                    .load(conn)
            })
            .unwrap_or_else(|| Ok(Vec::new()))
    }

    pub fn get_team(conn: &mut PgConnection, team_id: Uuid) -> QueryResult<Option<Team>> {
        use crate::db::schema::teams::dsl::*;
        debug!("Get team with id {}", team_id);
        teams
            .filter(id.eq(team_id))
            .select(Team::as_select())
            .first::<Team>(conn)
            .optional()
    }

    pub fn delete_team(conn: &mut PgConnection, team_id: Uuid) -> QueryResult<usize> {
        diesel::delete(teams::table.filter(teams::id.eq(team_id))).execute(conn)
    }
}
