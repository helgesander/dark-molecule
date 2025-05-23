extern crate diesel;

use std::env;
use std::sync::Arc;

use actix_cors::Cors;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer};
use diesel::prelude::*;
use dotenv::dotenv;
use env_logger::Env;
use log::info;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

mod db;
mod dtos;
mod handlers;
mod middleware;
mod models;
mod routes;
mod services;
mod utils;

use crate::db::schema::users::dsl::*;
use crate::dtos::db::UserForm;
use crate::models::user::User;
use crate::routes::init_routes;
use crate::services::scanner;
use crate::utils::config::CONFIG;
use crate::utils::errors::{AppError, AppErrorJson};
use crate::utils::hash_password;

fn create_admin_user(conn: &mut PgConnection) -> Result<(), AppError> {
    // Check if users table is empty
    let user_count = users.count().get_result::<i64>(conn)?;
    if user_count == 0 {
        let new_password: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(12)
            .map(char::from)
            .collect();

        let admin_user = UserForm {
            email: "admin@example.com".to_string(),
            password: hash_password(&new_password)?,
            is_admin: Some(true),
            first_name: None,
            last_name: None,
            username: "admin".to_string(),
        };

        User::create_user(conn, &admin_user)?;
        info!(
            "Admin user created: email: {} password: {}",
            admin_user.email, new_password
        );
    }
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Настраиваем более подробное логирование
    env_logger::Builder::from_env(Env::default().default_filter_or("debug"))
        .format_timestamp_millis()
        .format_module_path(false)
        .format_target(false)
        .init();

    let pool = db::establish_connection();
    let scanner_service = Arc::new(scanner::ScannerService::new(&CONFIG, pool.clone()));

    create_admin_user(&mut pool.get().unwrap()).unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(scanner_service.clone()))
            .wrap(Logger::new(
                "%a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T",
            ))
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    CONFIG.secret_key.clone(),
                )
                .cookie_name("session".parse().unwrap())
                .cookie_secure(false)
                .build(),
            )
            .configure(init_routes)
            .default_service(web::route().to(|| async {
                HttpResponse::NotFound().json(AppErrorJson {
                    status: 404,
                    error: "Not Found",
                })
            }))
    })
    .bind((CONFIG.server.host.as_str(), CONFIG.server.port))?
    .run()
    .await
}
