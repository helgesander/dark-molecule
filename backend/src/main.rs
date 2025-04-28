extern crate diesel;

use crate::routes::init_routes;
use crate::utils::config::AppConfig;
use crate::utils::errors::AppErrorJson;
use actix_cors::Cors;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::{Key, SameSite};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use env_logger::Env;
use std::env;
use std::time::Duration;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

mod db;
mod dtos;
mod handlers;
mod middleware;
mod models;
mod routes;
// mod services;
mod utils;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // use self::db::schema::users::dsl::*;
    #[cfg(debug_assertions)]
    {
        dotenv().ok();
        let config = AppConfig {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            port: 8000,
            log_level: "debug".to_string(),
            secret_key: Key::from("428742874djkjkdfsdhfhhkjdchsjkdhfkjshdfjkshdjfhsdjfh2873894728934728dajkshdkjahdkjahdjkahdjkahsjkdhajsdhajkhsdjakhsd27349287842742874387ajsdhkajshdjahdjkahdjkahsdjhajsdh23728734928734".as_bytes()),
            templates_path: "./templates".to_string(),
            scans_path: "./scans".to_string()
        };
    }

    let config = AppConfig::new().expect("Can't load config");
    env_logger::init_from_env(Env::default().default_filter_or(config.log_level.clone()));

    let pool = db::establish_connection();

    // let nuclei_service = NucleiService::new(config.scans_path.clone());
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(config.clone()))
            // .app_data(Data::new(nuclei_service.clone()))
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    config.secret_key.clone(),
                )
                .cookie_name("session".parse().unwrap())
                .cookie_secure(false) // В development можно false, в production должно быть true
                // .cookie_http_only(true)
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
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
