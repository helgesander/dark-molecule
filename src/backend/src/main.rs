use actix_web::{web, App, HttpServer, Responder};
use mongodb::Client;
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use log::info;
use dotenv::dotenv;

mod handlers;
mod routes;
mod models;
mod dto;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mongodb_uri = "mongodb://localhost:27017";
    let mut client_options = ClientOptions::parse(mongodb_uri).await.unwrap();
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let client = Client::with_options(client_options).unwrap();
    client.database("main").drop(None).await.unwrap();
    info!("Connected to mongodb... Pu-pu-pu...");
    HttpServer::new(|| {
        App::new()
            .configure(routes::init_routes)
    })
    .bind(("0.0.0.0", 1337))?
    .run()
    .await
}