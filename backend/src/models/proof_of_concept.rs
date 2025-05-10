use crate::db::schema;
use crate::dtos::handlers::ProofOfConceptForm;
use crate::{db::schema::proof_of_concepts, models::project::Project};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = schema::proof_of_concepts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProofOfConcept {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub code: String,
    pub issue_id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = schema::proof_of_concepts)]
pub struct NewProofOfConcept {
    pub title: String,
    pub description: String,
    pub code: String,
    pub issue_id: Uuid,
}

#[derive(Serialize, ToSchema)]
pub struct PocMetadata {
    pub description: String,
    // TODO: add hosts maybe
}

pub struct PocData {
    pub data: Vec<u8>,
    pub content_type: String,
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
            description: poc_metadata,
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
            content_type: form.content_type.clone(),
        };
        diesel::insert_into(proof_of_concepts)
            .values(&new_poc)
            .get_result::<ProofOfConcept>(conn)
    }

    pub fn get_poc_data(conn: &mut PgConnection, poc_id: i32) -> QueryResult<PocData> {
        use crate::db::schema::proof_of_concepts::dsl::*;
        let (poc_data, poc_content_type) = proof_of_concepts
            .filter(id.eq(poc_id))
            .select((data, content_type))
            .first::<(Vec<u8>, String)>(conn)?;

        Ok(PocData {
            data: poc_data,
            content_type: poc_content_type,
        })
    }
}
