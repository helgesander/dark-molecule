use yew::prelude::*;
use crate::api::{ApiClient, Project};
use uuid::Uuid;
use crate::components::project_sidebar::ProjectSidebar;
use crate::components::project_hosts::ProjectHosts;
use crate::components::project_issues::ProjectIssues;
use crate::components::project_reports::ProjectReports;
use crate::components::project_services::ProjectServices;
use crate::components::project_settings::ProjectSettings;
use crate::debug_log;

#[derive(Properties, PartialEq)] 
pub struct ProjectPageProps {
    pub project_id: Uuid,
    pub active_tab: Option<String>,
}

#[function_component(ProjectPage)]
pub fn project_page(props: &ProjectPageProps) -> Html {
    let project = use_state(|| None::<Project>);
    let error = use_state(|| String::new());
    let active_tab = use_state(|| props.active_tab.clone().unwrap_or("hosts".to_string()));
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
                debug_log!("Fetching project data for ID: {}", project_id);
                match ApiClient::get().get_full_project(project_id).await {
                    Ok(data) => {
                        debug_log!("Project data received: {}", &data.name);
                        project.set(Some(data));
                    }
                    Err(e) => {
                        debug_log!("Error loading project: {}", &e);
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
                        t if t == "hosts" => html! { <ProjectHosts hosts={project.hosts.clone()} project_id={project.id}/> },
                        t if t == "issues" => html! { <ProjectIssues issues={project.issues.clone()} project_id={project.id} /> },
                        t if t == "reports" => html! { <ProjectReports reports={project.reports.clone().unwrap_or_default()} project_id={project.id} /> },
                        // t if t == "services" => html! { <ProjectServices services={project.services.clone().unwrap_or_default()} /> },
                        // t if t == "settings" => html! { <ProjectSettings project_id={project.id} /> },
                        _ => html! { <div>{"Настройки проекта"}</div> },
                    }}
                </main>
            } else if !error.is_empty() {
                <div class="error-message">{error.to_string()}</div>
            } else {
                <div class="loading">{"Загрузка проекта..."}</div>
            }
        </div>
    }
} 