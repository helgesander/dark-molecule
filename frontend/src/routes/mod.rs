use yew_router::prelude::*;
use yew::prelude::*;

mod admin;
mod profile;
mod project;

#[derive(Clone, Routable, PartialEq)]
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
    MainPage
}

pub fn switch_main(route: MainRoute) -> Html {
    match route {
        MainRoute::Projects => html! { <h1>{ "Projects" }</h1> },
        MainRoute::ProjectRoot | MainRoute::Project => html! { <Switch<project::ProjectRoute> render={ project::switch_project }/>},
        MainRoute::Login => html! {<h1>{"Login Page"}</h1>},
        MainRoute::Register => html! {<h1>{"Register Page"}</h1>},
        MainRoute::Admin | MainRoute::AdminRoot => html! { <Switch<admin::AdminRoute> render={ admin::switch_admin }/> },
        MainRoute::Profile => html! {<h1>{"Profile Page"}</h1>},
        MainRoute::MainPage => html! {<h1> {"Main Page"} </h1>}
    }
}