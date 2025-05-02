use yew::prelude::*;
use gloo::console::log;
use yew_router::prelude::*;
use crate::routes::project::ProjectRoute;
use crate::api::{ApiClient, ProjectOverview};
use uuid::Uuid;

#[function_component(ProjectsPage)]
pub fn projects_page() -> Html {
    let projects = use_state(|| Vec::<ProjectOverview>::new());
    let error = use_state(|| String::new());
    let navigator = use_navigator().unwrap();

    {
        let projects = projects.clone();
        let error = error.clone();
        use_effect_with_deps(move |_| {
            let projects = projects.clone();
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                log!("Fetching projects...");
                match ApiClient::get().get_projects().await {
                    Ok(data) => {
                        log!("Received projects:", format!("{:?}", data));
                        projects.set(data);
                    }
                    Err(e) => {
                        log!("Error fetching projects:", &e);
                        if e == "unauthorized" {
                            error.set("Необходима авторизация".to_string());
                        } else {
                            let error_message = format!("Ошибка при загрузке проектов: {}", e);
                            error.set(error_message);
                        }
                    }
                }
            });
            || {}
        }, ());
    }

    let on_project_click = {
        let navigator = navigator.clone();
        Callback::from(move |id: Uuid| {
            log!("Clicked on project:", id.to_string());
            navigator.push(&ProjectRoute::Project { id });
        })
    };

    let on_create_click = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&ProjectRoute::CreateProject);
        })
    };

    html! {
        <>
            <div class="content">
                <div class="project-list">
                    <div class="projects-header">
                        <h1>{"Проекты"}</h1>
                        <button onclick={on_create_click} class="button">{"Создать проект"}</button>
                    </div>
                    if !error.is_empty() {
                        <div class="error-message">{error.to_string()}</div>
                    }
                    if projects.is_empty() {
                        <div class="empty-message">{"Нет доступных проектов"}</div>
                    } else {
                        <div class="projects-grid">
                            {for projects.iter().map(|project| {
                                let id = project.id;
                                let on_click = on_project_click.clone();
                                html! {
                                    <div class="project-card" onclick={Callback::from(move |_| on_click.emit(id))}>
                                        <h3>{&project.name}</h3>
                                        {if let Some(scope) = &project.scope {
                                            html! { <p class="description">{scope}</p> }
                                        } else {
                                            html! {}
                                        }}
                                    </div>
                                }
                            })}
                        </div>
                    }
                </div>
            </div>
        </>
    }
} 