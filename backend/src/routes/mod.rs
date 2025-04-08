use actix_web::web;
use crate::handlers::{user_handlers, project_handlers};

fn init_project_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/projects")
            .service(project_handlers::get_projects_handler)
            .service(project_handlers::get_project_handler)
            .service(project_handlers::create_project_handler)
    );
}

// fn init_report_routes(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::scope("/reports")
//         .service(report_handlers::create_report_handler)
//         .service(report_handlers::get_report_handler)
//     );
// }

fn init_auth_routes(cfg: &mut web::ServiceConfig) {
    todo!()
}

fn init_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(user_handlers::create_user_handler)
            .service(user_handlers::get_users_handler)
            .service(user_handlers::delete_user_handler)
            .service(user_handlers::get_user_handler)
    );
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(init_user_routes)
            .configure(init_project_routes)
            // .configure(init_report_routes)
    );
}