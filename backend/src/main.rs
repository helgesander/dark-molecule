extern crate diesel;

use std::env;
use std::sync::Arc;
use std::time::Duration;
use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use env_logger::Env;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::routes::init_routes;
use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_web::cookie::Key;
use dotenv::dotenv;
use crate::utils::ResponseJson;
use crate::utils::config;
use crate::utils::config::AppConfig;

mod handlers;
mod routes;
mod models;
mod utils;
mod db;
mod dtos;
mod middleware;
mod services;

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
        };
    }

    let config = AppConfig::new().expect("Can't load config");
    env_logger::init_from_env(Env::default().default_filter_or(config.log_level.clone()));

    let manager = ConnectionManager::<PgConnection>::new(config.database_url.clone());

    let pool = Pool::builder()
        .max_size(15)
        .connection_timeout(Duration::from_secs(30))
        .build(manager).expect("Can't create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(config.clone()))
            .wrap(Logger::default())
            .wrap(SessionMiddleware::builder(CookieSessionStore::default(), config.secret_key.clone())
                .cookie_name("session".parse().unwrap())
                // .cookie_max_age(time::Duration::hours(2)) // TODO: try it later
                .build())
            .configure(init_routes)
            .default_service(
                web::route()
                    .to(|| async {
                        HttpResponse::NotFound().json(ResponseJson {status: 404, message: "Not Found"})
                    })
            )
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}