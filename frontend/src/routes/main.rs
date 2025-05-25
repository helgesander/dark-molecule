use yew_router::prelude::*;
use yew::prelude::*;
use crate::pages::{
    main::MainPage,
    login::LoginPage,
    not_found::NotFoundPage,
    register::RegisterPage,
    projects::ProjectsPage,
    profile::ProfilePage,
    admin::AdminPage
};

use crate::routes::{
    project::{ProjectRoute, switch_project},
    admin::{AdminRoute, switch_admin}
};

#[derive(Clone, Debug, Routable, PartialEq)]
pub enum MainRoute {
    #[at("/projects")]
    Projects,
    #[at("/project")]
    ProjectRoot,
    #[at("/project/*")]
    Project,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/admin")]
    AdminRoot,
    #[at("/admin/*")]
    Admin,
    #[at("/profile")]
    Profile,
    #[at("/")]
    MainPage,
    #[at("/404")]
    #[not_found]
    NotFound
}

pub fn switch_main(route: MainRoute) -> Html {
    match route {
        MainRoute::Projects => html! {<ProjectsPage />},
        MainRoute::ProjectRoot | MainRoute::Project => html! { <Switch<ProjectRoute> render={ switch_project }/>},
        MainRoute::Login => html! {<LoginPage />},
        MainRoute::Register => html! {<RegisterPage />},
        MainRoute::Admin | MainRoute::AdminRoot => html! { <AdminPage /> },
        MainRoute::Profile => html! {<ProfilePage />},
        MainRoute::MainPage => html! {<MainPage />},
        MainRoute::NotFound => html! {<NotFoundPage />}
    }
}