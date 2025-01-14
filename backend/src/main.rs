extern crate diesel;

use std::env;
use actix_web::{App, HttpServer, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;
use diesel::prelude::*;


mod handlers;
mod routes;
mod models;
mod utils;
mod db;
mod dtos;

fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use self::db::schema::users::dsl::*;
    #[cfg(debug_assertions)]
    {
        use dotenv::dotenv;
        dotenv().ok();
    }
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let connection = &mut establish_connection();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(routes::init_routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}