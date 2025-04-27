use crate::db::schema;
use diesel::prelude::*;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Insertable, Deserialize, Validate, AsChangeset, Debug)]
#[diesel(table_name = schema::users)]
pub struct UserForm {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[validate(length(min = 1))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Insertable, Deserialize, Debug, AsChangeset)]
#[diesel(table_name = schema::teams)]
pub struct TeamForm {
    name: String,
    description: Option<String>,
    admin_id: Uuid,
}
