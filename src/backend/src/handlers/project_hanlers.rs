
use actix_web::{HttpResponse, Responder};

fn get_projects() -> impl Responder {
    HttpResponse::Ok().body("Getting products, just wait...")
}

// TODO: implement all project handlers
