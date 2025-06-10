use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::schema::{hosts, projects};
use crate::dtos::handlers::HostForm;
use crate::models::project::Project;

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
    pub(crate) hostname: Option<String>,
    pub(crate) ip_address: String,
    pub(crate) project_id: Uuid,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct HostResponse {
    pub id: i32,
    pub hostname: Option<String>,
    pub ip_address: String,
}

impl Host {
    pub fn get_hosts_by_project_id(
        conn: &mut PgConnection,
        id_project: Uuid,
    ) -> QueryResult<Vec<HostResponse>> {
        use crate::db::schema::hosts::dsl::*;

        let project = match projects::table
            .find(id_project)
            .select(Project::as_select())
            .first(conn)
            .optional()?
        {
            Some(p) => p,
            None => return Ok(Vec::new()),
        };

        let selected_hosts = Host::belonging_to(&project)
            .select((id, hostname, ip_address))
            .load::<(i32, Option<String>, String)>(conn)?;

        let mut result = Vec::with_capacity(selected_hosts.len());
        for (host_id, host, ip) in selected_hosts {
            result.push(HostResponse {
                id: host_id,
                hostname: host,
                ip_address: ip,
            });
        }

        Ok(result)
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

    pub fn get_host(conn: &mut PgConnection, host_id: i32) -> QueryResult<Option<Host>> {
        use crate::db::schema::hosts::dsl::*;
        hosts
            .filter(id.eq(host_id))
            .select(Host::as_select())
            .first(conn)
            .optional()
    }

    pub fn get_host_by_ip(conn: &mut PgConnection, ip: String) -> QueryResult<Option<Host>> {
        use crate::db::schema::hosts::dsl::*;
        hosts
            .filter(ip_address.eq(ip))
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

    pub fn create_hosts(
        conn: &mut PgConnection,
        forms: Vec<HostForm>,
        id_project: Uuid,
    ) -> QueryResult<Vec<Host>> {
        use crate::db::schema::hosts::dsl::*;
        let mut new_hosts_vec: Vec<NewHost> = Vec::new();
        for form in forms {
            new_hosts_vec.push(NewHost {
                hostname: form.hostname,
                ip_address: form.ip_address,
                project_id: id_project,
            });
        }
        diesel::insert_into(hosts)
            .values(new_hosts_vec)
            .get_results::<Host>(conn)
    }
}
