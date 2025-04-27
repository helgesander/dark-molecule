use yew_router::prelude::*;
use yew::prelude::*;
use crate::pages::main::MainPage;
use crate::pages::login::LoginPage;
use crate::pages::not_found::NotFoundPage;
use crate::pages::register::RegisterPage;

mod admin;
mod profile;
mod project;

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
        MainRoute::Projects => html! { <h1>{ "Projects" }</h1> },
        MainRoute::ProjectRoot | MainRoute::Project => html! { <Switch<project::ProjectRoute> render={ project::switch_project }/>},
        MainRoute::Login => html! {<LoginPage />},
        MainRoute::Register => html! {<RegisterPage />},
        MainRoute::Admin | MainRoute::AdminRoot => html! { <Switch<admin::AdminRoute> render={ admin::switch_admin }/> },
        MainRoute::Profile => html! {<h1>{"Profile Page"}</h1>},
        MainRoute::MainPage => html! {<MainPage />},
        MainRoute::NotFound => html! {<NotFoundPage />}
    }
}