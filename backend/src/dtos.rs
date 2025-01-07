use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::sql_types::Date;
use crate::db::schema;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Insertable, Deserialize)]
#[diesel(table_name = schema::users)]
pub struct UserForm<'a> {
    first_name: Option<&'a str>,
    last_name: Option<&'a str>,
    username: &'a str,
    email: &'a str,
    password: &'a str,
    is_admin: bool // WARNING! SECURITY RISK!!!
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
