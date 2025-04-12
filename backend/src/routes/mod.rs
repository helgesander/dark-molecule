use actix_web::middleware::from_fn;
use actix_web::web;
use crate::handlers::{user_handlers, project_handlers, auth_handlers, admin_handlers, report_handlers, issue_handlers, scan_handlers, team_handlers};
use crate::middleware::auth::auth_middleware;

// TODO: add auth wrappers for all routes
fn init_project_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/project")
            .service(project_handlers::get_projects_handler)
            .service(project_handlers::get_project_handler)
            .service(project_handlers::create_project_handler)
    );
}

fn init_report_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/report")
        .service(report_handlers::create_report_handler)
        .service(report_handlers::get_report_handler)
    );
}

fn init_issue_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/issue")
            .service(issue_handlers::get_issue_handler)
            .service(issue_handlers::create_issue_handler)
    );
}


fn init_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(auth_handlers::auth_handler)
    );
}

fn init_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            // .wrap(from_fn(auth_middleware))
            .service(user_handlers::create_user_handler)
            .service(user_handlers::get_users_handler)
            .service(user_handlers::delete_user_handler)
            .service(user_handlers::get_user_handler)
    );
}

fn init_team_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/team")
            .service(team_handlers::create_team_handler)
            .service(team_handlers::get_teams_handler)
    );
}

fn init_admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .service(admin_handlers::get_admin_settings_handler)
    );
}

fn init_scan_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/scan")
            .service(scan_handlers::get_scan_handler)
            .service(scan_handlers::create_scan_handler)
    );
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(init_user_routes)
            .configure(init_project_routes)
            .configure(init_auth_routes)
            .configure(init_admin_routes)
            .configure(init_scan_routes)
            .configure(init_team_routes)
    );
}