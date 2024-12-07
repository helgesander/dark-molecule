use actix_web::{web, App, HttpServer, Responder};

mod handlers;
mod routes;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _json_conf = web::JsonConfig::default()
        .limit(4096);
    HttpServer::new(|| {
        App::new()
            // .app_data(json_conf)
            .configure(routes::init_routes)
    })
    .bind(("0.0.0.0", 1337))?
    .run()
    .await
}