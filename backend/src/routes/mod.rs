use actix_web::middleware::from_fn;
use actix_web::web;

use crate::handlers::{
    auth_handlers, project_handlers, team_handlers, template_handlers,
    user_handlers,
};
use crate::middleware::auth::auth_middleware;

// TODO: add auth wrappers for all routes
fn init_project_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/project")
            // .wrap(from_fn(auth_middleware))
            .service(project_handlers::get_projects_handler)
            .service(project_handlers::get_project_handler)
            .service(project_handlers::create_project_handler)
            .service(project_handlers::get_issues_handler)
            .service(project_handlers::get_hosts_handler)
            .service(project_handlers::get_host_handler)
            .service(project_handlers::delete_issue_handler)
            .service(project_handlers::update_issue_handler)
            .service(project_handlers::create_host_handler)
            .service(project_handlers::update_host_handler)
            .service(project_handlers::delete_host_handler)
            .service(project_handlers::create_issue_handler)
            .service(project_handlers::get_issue_handler)
            .service(project_handlers::create_report_handler)
            .service(project_handlers::get_report_previews_for_project_handler)
            .service(project_handlers::get_report_handler)
            .service(project_handlers::get_scan_result_handler)
            .service(project_handlers::start_scan_handler),
    );
}

fn init_template_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/template")
            // .wrap(from_fn(auth_middleware))
            .service(template_handlers::get_template_handler)
            .service(template_handlers::create_template_handler)
    )
        .service(
            web::scope("/templates")
                // .wrap(from_fn(auth_middleware))
                .service(template_handlers::get_templates_preview_handler)
        );
}

fn init_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(auth_handlers::auth_handler)
            .service(auth_handlers::logout_handler),
    );
}

fn init_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            // .wrap(from_fn(auth_middleware))
            .service(user_handlers::create_user_handler)
            .service(user_handlers::get_users_handler)
            .service(user_handlers::delete_user_handler)
            .service(user_handlers::get_user_handler),
    );
}

fn init_team_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/team")
            // .wrap(from_fn(auth_middleware))
            .service(team_handlers::create_team_handler)
            .service(team_handlers::get_teams_handler)
            .service(team_handlers::get_team_handler),
    );
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(init_user_routes)
            .configure(init_project_routes)
            .configure(init_auth_routes)
            .configure(init_team_routes)
            .configure(init_template_routes),
    );
}
