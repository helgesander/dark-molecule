use diesel::{Identifiable, Insertable, PgConnection, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::dtos::handlers::ProofOfConceptForm;

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

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::db::schema::proof_of_concepts)]
pub struct NewProofOfConcept {
    description: String,
    data: Vec<u8>,
    issue_id: Uuid
}

impl ProofOfConcept {
    pub fn get_pocs_by_issue_id(conn: &mut PgConnection, project_id: Uuid) {
        unimplemented!()
    }

    pub fn create_poc(conn: &mut PgConnection, data: &ProofOfConceptForm) {
        unimplemented!()
    }
}