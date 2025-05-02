use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::project::ProjectRoute;
use crate::api::{ApiClient, Project};
use uuid::Uuid;
use crate::components::project_sidebar::ProjectSidebar;
use crate::components::project_hosts::ProjectHosts;
use crate::components::project_issues::ProjectIssues;

#[derive(Properties, PartialEq)]
pub struct ProjectPageProps {
    pub project_id: Uuid,
}

#[function_component(ProjectPage)]
pub fn project_page(props: &ProjectPageProps) -> Html {
    let project = use_state(|| None::<Project>);
    let error = use_state(|| String::new());
    let active_tab = use_state(|| "hosts".to_string());
    let on_tab_select = {
        let active_tab = active_tab.clone();
        Callback::from(move |tab: String| active_tab.set(tab))
    };

    {
        let project = project.clone();
        let error = error.clone();
        let project_id = props.project_id;
        use_effect_with_deps(move |_| {
            let project = project.clone();
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match ApiClient::get().get_full_project(project_id).await {
                    Ok(data) => {
                        project.set(Some(data));
                    }
                    Err(e) => {
                        let error_message = format!("Ошибка при загрузке проекта: {}", e);
                        error.set(error_message);
                    }
                }
            });
            || {}
        }, ());
    }

    html! {
        <div class="project-layout">
            if let Some(project) = &*project {
                <ProjectSidebar active_tab={(*active_tab).clone()} on_tab_select={on_tab_select} project_name={project.name.clone()}/>
                <main class="project-main">
                    if !error.is_empty() {
                        <div class="error-message">{error.to_string()}</div>
                    }
                    {match &*active_tab {
                        t if t == "hosts" => html! { <ProjectHosts hosts={project.hosts.clone()} /> },
                        t if t == "issues" => html! { <ProjectIssues issues={project.issues.clone()} /> },
                        t if t == "reports" => html! { <div>{"Здесь будут отчеты проекта"}</div> },
                        t if t == "services" => html! { <div>{"Здесь будут сервисы проекта"}</div> },
                        t if t == "settings" => html! { <div>{"Настройки проекта"}</div> },
                        _ => html! { <div>{"Настройки проекта"}</div> },
                    }}
                </main>
            } else {
                <div>{"Загрузка..."}</div>
            }
        </div>
    }
} 