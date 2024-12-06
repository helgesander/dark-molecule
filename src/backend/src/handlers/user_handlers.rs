use actix_web::{HttpResponse, Responder};

fn get_user() -> impl Responder {
    HttpResponse::Ok().body("Getting user, just wait...")
}
