use chrono::{NaiveDate, NaiveDateTime};
use diesel::{Identifiable, PgConnection, QueryResult, Queryable, Selectable};
use crate::db::schema::proof_of_concepts::dsl::proof_of_concepts;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::db::schema::proof_of_concepts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Issue))]
#[diesel(primary_key(id))]
pub struct ProofOfConcept {
    pub id: Uuid,
    pub issue_id: Uuid,
    pub description: String,
    pub data: Vec<u8>,
}

impl ProofOfConcept {
    pub fn get_pocs_by_issue_id(conn: &mut PgConnection, project_id: Uuid) {
        unimplemented!()
    }
}