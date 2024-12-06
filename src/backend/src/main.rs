use actix_web::{error, get, post, web::{self, scope}, App, Error, HttpResponse, HttpServer, Responder};
// use crate::


mod handlers;
mod routes;

async fn get_projects() -> impl Responder {
    HttpResponse::Ok().body("In develop")
    // TODO: implement get projects
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let json_conf = web::JsonConfig::default()
        .limit(4096);
        // .error_handler(|err, req| {
        //     error::InternalError::from_response(err, HttpResponse::Conflict().into().into())
        // });
    HttpServer::new(|| {
        App::new()
            .app_data(json_conf)
            .configure()
    })
    .bind(("0.0.0.0", 1337))?
    .run()
    .await
}