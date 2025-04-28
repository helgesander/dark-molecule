use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::projects::ProjectsPage;

#[derive(Clone, Routable, PartialEq)]
pub enum ProjectRoute {
    #[at("/projects")]
    Projects,
    #[at("/project/{:id}")]
    Project,
    #[at("/project/{:id}/edit")]
    ProjectEdit
}

pub fn switch_project(route: ProjectRoute) -> Html {
    match route {
        ProjectRoute::Projects => html! {<ProjectsPage />},
        ProjectRoute::Project => html! { <h1>{"Edit Project"}</h1> },
        ProjectRoute::ProjectEdit => html! { <h1>{"Project"}</h1> }
    }
} 