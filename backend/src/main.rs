extern crate diesel;

use std::env;
use std::time::Duration;
use actix_web::{App, HttpServer, Responder};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use env_logger::Env;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::routes::init_routes;

mod handlers;
mod routes;
mod models;
mod utils;
mod db;
mod dtos;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use self::db::schema::users::dsl::*;
    let mut log_level = "info";
    #[cfg(debug_assertions)]
    {
        use dotenv::dotenv;
        dotenv().ok();
        log_level = "debug";
    }
    env_logger::init_from_env(Env::default().default_filter_or(log_level));
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .max_size(15)
        .connection_timeout(Duration::from_secs(30))
        .build(manager).expect("Can't create pool");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(Logger::default())
            .configure(init_routes)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}