use diesel::{Identifiable, PgConnection, QueryDsl, QueryResult, Queryable, RunQueryDsl, Selectable, SelectableHelper, Table};
use diesel::associations::HasTable;
use serde::Serialize;
use uuid::Uuid;
use crate::db::schema::report_templates::dsl::report_templates;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable, Serialize)]
#[diesel(table_name = crate::db::schema::report_templates)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Team))]
pub struct ReportTemplate {
    pub id: Uuid,
    pub team_id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub filename: String
}


impl ReportTemplate {
    pub fn get_report_templates(conn: &mut PgConnection) -> QueryResult<Vec<ReportTemplate>> {
        report_templates::table()   .load(conn)
    }

    pub fn create_report_template(conn: &mut PgConnection) -> QueryResult<ReportTemplate> {
        unimplemented!()
    }
}