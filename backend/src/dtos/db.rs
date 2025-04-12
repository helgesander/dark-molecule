use chrono::NaiveDate;
use diesel::prelude::*;
use crate::db::schema;
use serde::Deserialize;
use uuid::Uuid;
use validator::{Validate};

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
#[diesel(table_name = schema::projects)]
pub struct ProjectForm {
    name: String,
    description: Option<String>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    folder: String,
    team_id: Uuid
}

#[derive(Insertable, Deserialize, Debug, AsChangeset)]
#[diesel(table_name = schema::teams)]
pub struct TeamForm {
    name: String,
    description: Option<String>,
    admin_id: Uuid
}

#[derive(Insertable, Deserialize, Debug, AsChangeset)]
#[diesel(table_name = schema::issues)]
pub struct IssueForm {
    name: String,
    description: String,
    mitigation: String,
    cvss: f64,
    project_id: Uuid
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = schema::hosts)]
pub struct HostForm {
    hostname: Option<String>,
    ip_address: String,
    project_id: Uuid
}