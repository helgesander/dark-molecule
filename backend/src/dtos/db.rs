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
    // pub is_admin: bool // WARNING! SECURITY RISK!!!
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = schema::projects)]
pub struct ProjectForm<'a> {
    name: &'a str,
    description: Option<&'a str>,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    folder: &'a str,
    team_id: Uuid
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = schema::teams)]
pub struct TeamForm<'a> {
    name: &'a str,
    description: Option<&'a str>,
    admin_id: Uuid
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = schema::issues)]
pub struct IssueForm<'a> {
    name: &'a str,
    description: &'a str,
    mitigation: &'a str,
    cvss: f64,
    project_id: Uuid
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = schema::hosts)]
pub struct HostForm<'a> {
    hostname: Option<&'a str>,
    ip_address: &'a str,
    project_id: Uuid
}