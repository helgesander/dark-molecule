use uuid::Uuid;
use diesel::prelude::*;
use crate::{
    db::schema::teams,
    models::user::User
};

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = teams)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
#[diesel(belongs_to(User))]
pub struct Team {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    admin_id: Uuid
}
