use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::db::schema::users;
use diesel::prelude::*;
use chrono::NaiveDate;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::db::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
pub struct User {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDate,
    pub is_admin: bool,
    pub is_active: bool,
}
