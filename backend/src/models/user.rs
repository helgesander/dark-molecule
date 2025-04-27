use crate::db::schema::users;
use crate::db::schema::users::dsl::*;
use crate::db::schema::users::id;
use crate::dtos::db::UserForm;
use crate::utils::FilterObjects;
use chrono::NaiveDate;
use diesel::prelude::*;
use log::debug;
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, Debug, Serialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
pub struct User {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDate,
    pub is_admin: bool,
    pub is_active: bool,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: String,
    pub email: String,
    pub create_at: NaiveDate,
    pub is_admin: bool,
    pub is_active: bool,
}

impl User {
    pub fn get_user_by_id(conn: &mut PgConnection, user_id: Uuid) -> QueryResult<Option<User>> {
        debug!("Select query for user with id {}", user_id);
        users
            .filter(id.eq(user_id))
            .select(User::as_select())
            .first(conn)
            .optional()
    }

    pub fn get_user_by_email(
        conn: &mut PgConnection,
        user_email: String,
    ) -> QueryResult<Option<User>> {
        // debug!("User with email {} try login", email);
        users
            .filter(email.eq(user_email))
            .select(User::as_select())
            .first(conn)
            .optional()
    }

    pub fn get_users(
        conn: &mut PgConnection,
        filter_data: &FilterObjects,
    ) -> QueryResult<Vec<UserResponse>> {
        debug!("size = {}, name = {}", filter_data.size, filter_data.name);
        let all_users = users
            .select(User::as_select())
            .filter(username.eq(filter_data.name.clone()))
            .limit(filter_data.size as i64)
            .order(created_at.asc())
            .load(conn)?;

        let mut result: Vec<UserResponse> = Vec::new();
        for user in all_users {
            result.push(user.to_response(conn)?)
        }
        Ok(result)
    }

    pub fn create_user(conn: &mut PgConnection, form: &UserForm) -> QueryResult<UserResponse> {
        debug!("Create user with data: {:?}", form);
        let user = diesel::insert_into(users::table)
            .values(form)
            .get_result::<User>(conn)?;
        Ok(user.to_response(conn)?)
    }

    pub fn update_user(
        conn: &mut PgConnection,
        form: &UserForm,
        user_id: Uuid,
    ) -> QueryResult<usize> {
        debug!("Update user with data: {:?}", form);
        diesel::update(users.filter(id.eq(user_id)))
            .set(form)
            .execute(conn)
    }

    pub fn delete_user(conn: &mut PgConnection, user_id: Uuid) -> QueryResult<usize> {
        conn.transaction(|conn| {
            let count = diesel::delete(users::table.filter(users::id.eq(user_id))).execute(conn)?;

            debug!("Deleted {} rows (before commit)", count);
            Ok(count)
        })
    }

    pub fn to_response(&self, conn: &mut PgConnection) -> QueryResult<UserResponse> {
        Ok(UserResponse {
            id: self.id,
            first_name: self.first_name.clone(),
            last_name: self.first_name.clone(),
            username: self.username.clone(),
            email: self.email.clone(),
            create_at: self.created_at,
            is_admin: false,
            is_active: false,
        })
    }
}
