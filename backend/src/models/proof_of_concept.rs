use crate::db::schema::issues;
use crate::dtos::handlers::ProofOfConceptForm;
use crate::models::issue::Issue;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::{Identifiable, Insertable, PgConnection, QueryResult, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, Debug, Serialize, Deserialize, Associations)]
#[diesel(table_name = crate::db::schema::proof_of_concepts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Issue))]
#[diesel(primary_key(id))]
pub struct ProofOfConcept {
    id: i32,
    issue_id: Uuid,
    description: String,
    data: Vec<u8>,
    mime_type: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::db::schema::proof_of_concepts)]
pub struct NewProofOfConcept {
    pub description: String,
    pub data: Vec<u8>,
    pub issue_id: Uuid,
    pub mime_type: String,
}

#[derive(Serialize)]
pub struct PocMetadata {
    pub description: String,
    // TODO: add hosts maybe
}

pub struct PocData {
    pub data: Vec<u8>,
    pub mime_type: String,
}

impl ProofOfConcept {
    pub fn get_pocs_by_issue_id(
        conn: &mut PgConnection,
        id_issue: Uuid,
    ) -> QueryResult<Vec<ProofOfConcept>> {
        use crate::db::schema::proof_of_concepts::dsl::*;
        issues::table
            .find(id_issue)
            .select(Issue::as_select())
            .first(conn)
            .optional()?
            .map(|issue| {
                ProofOfConcept::belonging_to(&issue)
                    .select(ProofOfConcept::as_select())
                    .load(conn)
            })
            .unwrap_or_else(|| Ok(Vec::new()))
    }

    pub fn get_poc(conn: &mut PgConnection, poc_id: i32) -> QueryResult<PocMetadata> {
        use crate::db::schema::proof_of_concepts::dsl::*;
        let poc_metadata = proof_of_concepts
            .filter(id.eq(poc_id))
            .select(description)
            .first::<String>(conn)?;
        Ok(PocMetadata {
            description: poc_metadata
        })
    }

    pub fn create_poc(
        conn: &mut PgConnection,
        form: &ProofOfConceptForm,
        id_issue: Uuid,
    ) -> QueryResult<ProofOfConcept> {
        use crate::db::schema::proof_of_concepts::dsl::*;
        let new_poc = NewProofOfConcept {
            description: form.description.clone(),
            data: form.data.clone(),
            issue_id: id_issue,
            mime_type: form.mime_type.clone(),
        };
        diesel::insert_into(proof_of_concepts)
            .values(&new_poc)
            .get_result::<ProofOfConcept>(conn)
    }

    pub fn get_poc_data(conn: &mut PgConnection,  poc_id: i32) -> QueryResult<PocData> {
        use crate::db::schema::proof_of_concepts::dsl::*;
        let (poc_data, poc_mime_type) = proof_of_concepts
            .filter(id.eq(poc_id))
            .select((data, mime_type))
            .first::<(Vec<u8>, String)>(conn)?;

        Ok(PocData {
            data: poc_data,
            mime_type: poc_mime_type,
        })
    }
}
