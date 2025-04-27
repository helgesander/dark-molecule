use crate::db::schema::projects;
use crate::dtos::handlers::HostForm;
use crate::{db::schema::hosts, models::project::Project};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Identifiable, Serialize, Selectable, Associations, PartialEq, Debug)]
#[diesel(table_name = hosts)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Project))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Host {
    pub id: i32,
    pub hostname: Option<String>,
    pub ip_address: String,
    pub project_id: Uuid,
}

#[derive(Insertable, Deserialize, AsChangeset, Debug)]
#[diesel(table_name = hosts)]
pub struct NewHost {
    hostname: Option<String>,
    ip_address: String,
    project_id: Uuid,
}

impl Host {
    pub fn get_hosts_by_project_id(
        conn: &mut PgConnection,
        id_project: Uuid,
    ) -> QueryResult<Vec<Host>> {
        projects::table
            .find(id_project)
            .select(Project::as_select())
            .first(conn)
            .optional()?
            .map(|project| {
                Host::belonging_to(&project)
                    .select(Host::as_select())
                    .load(conn)
            })
            .unwrap_or_else(|| Ok(Vec::new()))
    }

    pub fn create_host(
        conn: &mut PgConnection,
        form: &HostForm,
        id_project: Uuid,
    ) -> QueryResult<Host> {
        use crate::db::schema::hosts::dsl::*;
        let new_host = NewHost {
            hostname: form.hostname.clone(),
            ip_address: form.ip_address.clone(),
            project_id: id_project,
        };
        diesel::insert_into(hosts)
            .values(new_host)
            .get_result::<Host>(conn)
    }

    pub fn get_host(conn: &mut PgConnection, id: i32) -> QueryResult<Option<Host>> {
        use crate::db::schema::hosts::dsl::*;
        hosts
            .filter(id.eq(id))
            .select(Host::as_select())
            .first(conn)
            .optional()
    }

    pub fn delete_host(conn: &mut PgConnection, host_id: i32) -> QueryResult<usize> {
        use crate::db::schema::hosts::dsl::*;
        conn.transaction(|conn| diesel::delete(hosts.filter(id.eq(host_id))).execute(conn))
    }

    pub fn update_host(
        conn: &mut PgConnection,
        form: &HostForm,
        id_project: Uuid,
        host_id: i32,
    ) -> QueryResult<usize> {
        use crate::db::schema::hosts::dsl::*;
        let new_host = NewHost {
            hostname: form.hostname.clone(),
            ip_address: form.ip_address.clone(),
            project_id: id_project,
        };
        diesel::update(hosts.filter(id.eq(host_id)))
            .set(&new_host)
            .execute(conn)
    }
}
