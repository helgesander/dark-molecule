use crate::db::schema::projects;
use crate::models::team::Team;
use uuid::Uuid;
use diesel::prelude::*;
use chrono::NaiveDate;

#[derive(Queryable, Selectable, Identifiable)]
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

