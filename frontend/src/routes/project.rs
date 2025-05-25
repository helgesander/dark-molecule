use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::projects::ProjectsPage;
use crate::pages::create_project::CreateProjectPage;
use crate::pages::project::ProjectPage;
use crate::pages::edit_issue::EditIssuePage;
use uuid::Uuid;

#[derive(Clone, Routable, PartialEq)]
pub enum ProjectRoute {
    #[at("/projects")]
    Projects,
    #[at("/project/:id")]
    Project { id: Uuid },
    #[at("/project/create")]
    CreateProject,
    #[at("/project/:id/issue/:issue_id")]
    EditIssue { id: Uuid, issue_id: Uuid },
}

pub fn switch_project(route: ProjectRoute) -> Html {
    match route {
        ProjectRoute::Projects => html! { <ProjectsPage /> },
        ProjectRoute::Project { id } => html! { <ProjectPage project_id={id} /> },
        ProjectRoute::CreateProject => html! { <CreateProjectPage /> },
        ProjectRoute::EditIssue { id, issue_id } => html! { <EditIssuePage project_id={id} issue_id={issue_id} /> },
    }
} 