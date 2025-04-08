use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::schema::users;
use diesel::prelude::*;
use chrono::NaiveDate;
use diesel::r2d2::PooledConnection;
use log::debug;
use crate::db::schema::users::dsl::*;
use crate::db::schema::users::id;
use crate::dtos::db::UserForm;
use crate::utils::hash_password;

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

impl User {
    pub fn get_user_by_id(conn: &mut PgConnection, user_id: Uuid) -> QueryResult<Option<User>> {
        debug!("Select query for user with id {}", user_id);
        users
            .filter(id.eq(user_id))
            .select(User::as_select())
            .first(conn)
            .optional()
    }

    // TODO: maybe change arg type of size to i64
    pub fn get_users(conn: &mut PgConnection, size: usize) -> QueryResult<Vec<User>> {
        debug!("Get {} users", size);
        users
            .select(User::as_select())
            .limit(size as i64)
            .order(created_at.asc())
            .load(conn)
    }

    pub fn create_user(conn: &mut PgConnection, form: &UserForm) -> QueryResult<User> {
        debug!("Create user with data: {:?}", form);
        let password_hash = hash_password(&form.password)
            .map_err(|e| diesel::result::Error::DeserializationError(Box::new(e)))?;
        diesel::insert_into(users::table)
            .values(form)
            .get_result::<User>(conn)
    }

    pub fn update_user(conn: &mut PgConnection, form: &UserForm, user_id: Uuid) -> QueryResult<usize> {
        debug!("Update user with data: {:?}", form);
        diesel::update(users.filter(id.eq(user_id)))
            .set(form)
            .execute(conn)
    }

    pub fn delete_user(conn: &mut PgConnection, user_id: Uuid) -> QueryResult<usize> {
        debug!("Delete user with id {}", user_id);
        diesel::delete(users.filter(id.eq(user_id))).execute(conn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::sql_query;
    use std::env;
    use diesel::sql_types::Text;
    use dotenv;
    use serde_json;
    use crate::db::schema::users::dsl::*;
    use pretty_assertions::{assert_eq, assert_str_eq};

    const TEST_JSON: &str = r#"{
            "first_name": null,
            "last_name": null,
            "username": "johndoe",
            "email": "john.doe@example.com",
            "password": "securePassword123!",
            "is_admin": true
        }"#;

    #[test]
    fn test_create_user() {
        dotenv::from_filename(".env.test").ok();
        let mut connection = crate::establish_connection();
        let new_user = serde_json::from_str::<UserForm>(TEST_JSON).unwrap();
        let result = User::create_user(&mut connection, &new_user);
        assert!(result.is_ok());
        let created_user = result.unwrap();
        assert_eq!(created_user.username, new_user.username);
        assert_eq!(created_user.email, new_user.email);

        sql_query("TRUNCATE users CASCADE;")
            .execute(&mut connection).expect("Can't truncate table users");
    }
    #[test]
    fn test_get_user_by_id() {
        dotenv::from_filename(".env.test").ok();
        let mut connection = crate::establish_connection();
        let data = serde_json::from_str::<UserForm>(TEST_JSON).unwrap();
        let created_user = User::create_user(&mut connection, &data).unwrap();
        let result = User::get_user_by_id(&mut connection, created_user.id);
        assert!(result.is_ok());
        let retrieved_user = result.unwrap();
        match retrieved_user {
            Some(user) => {
                assert_eq!(user.id, created_user.id);
                assert_str_eq!(user.email.as_str() , &created_user.email);
            },
            None => panic!("User does not exist"),
        }

        sql_query("TRUNCATE users CASCADE;")
            .execute(&mut connection).expect("Can't truncate table users");
    }

    #[test]
    #[ignore]
    fn test_update_user() {
        dotenv::from_filename(".env.test").ok();

        let mut connection = crate::establish_connection();
        let mut updates = serde_json::from_str::<UserForm>(TEST_JSON).unwrap();
        let created_user = User::create_user(&mut connection, &updates).expect("Can't create user in tests");
        updates.first_name = Some("Updated");

        let result = User::update_user(&mut connection, &updates, created_user.id);
        assert!(result.is_ok());

        let updated_user: User = users.filter(id.eq(created_user.id)).first(&mut connection).expect("Failed to fetch updated user");
        assert_eq!(updated_user.first_name.unwrap().as_str(), updates.first_name.unwrap());
        
        // sql_query("TRUNCATE users CASCADE;")
        //     .execute(&mut connection).expect("Can't truncate table users");
    }

        #[test]
        fn test_delete_user() {
            dotenv::from_filename(".env.test").ok();
            let mut connection = crate::establish_connection();
            let data = serde_json::from_str::<UserForm>(TEST_JSON);
            let created_user = User::create_user(&mut connection, &data.unwrap()).expect("Can't create user in tests");

            let result = User::delete_user(&mut connection, created_user.id);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 1);

            let count = users.count().get_result::<i64>(&mut connection).unwrap();
            assert_eq!(count, 0);

            sql_query("TRUNCATE users CASCADE;")
                .execute(&mut connection).expect("Can't truncate table users");
        }
    }
