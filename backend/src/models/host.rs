use uuid::Uuid;
use diesel::prelude::*;
use crate::{
    db::schema::hosts,
    models::project::Project
};

#[derive(Queryable, Identifiable, Selectable)]
#[diesel(table_name = hosts)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Project))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Host {
    pub id: i32,
    pub hostname: Option<String>,
    pub ip_address: String,
    pub project_id: Uuid
}
