extern crate diesel;

use actix_web::{App, HttpServer, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;

mod handlers;
mod routes;
mod models;
mod utils;
mod db;
mod dtos;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(routes::init_routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}