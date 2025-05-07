use yew::prelude::*;
use gloo::console::log;
use crate::api::{ApiClient, Team};
use uuid::Uuid;
use yew_router::prelude::*;
use crate::routes::project::ProjectRoute;
use web_sys::HtmlElement;

#[derive(serde::Serialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub scope: Option<String>,
    pub team_id: Uuid,
}

#[function_component(CreateProjectPage)]
pub fn create_project_page() -> Html {
    let teams = use_state(|| Vec::<Team>::new());
    let selected_team = use_state(|| None::<Uuid>);
    let project_name = use_state(|| String::new());
    let project_scope = use_state(|| String::new());
    let error = use_state(|| String::new());
    let navigator = use_navigator().unwrap();

    {
        let teams = teams.clone();
        let error = error.clone();
        use_effect_with_deps(move |_| {
            let teams = teams.clone();
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                log!("Fetching teams...");
                match ApiClient::get().get_teams().await {
                    Ok(data) => {
                        log!("Received teams:", format!("{:?}", data));
                        teams.set(data);
                    }
                    Err(e) => {
                        log!("Error fetching teams:", &e);
                        let error_message = format!("Ошибка при загрузке команд: {}", e);
                        error.set(error_message);
                    }
                }
            });
            || {}
        }, ());
    }

    let on_team_change = {
        let selected_team = selected_team.clone();
        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlElement>().unwrap();
            if let Some(value) = input.get_attribute("value") {
                if let Ok(id) = Uuid::parse_str(&value) {
                    selected_team.set(Some(id));
                }
            }
        })
    };

    let on_name_change = {
        let project_name = project_name.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
            project_name.set(input.value());
        })
    };

    let on_scope_change = {
        let project_scope = project_scope.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
            project_scope.set(input.value());
        })
    };

    let on_submit = {
        let project_name = project_name.clone();
        let project_scope = project_scope.clone();
        let selected_team = selected_team.clone();
        let error = error.clone();
        let navigator = navigator.clone();
        Callback::from(move |_| {
            log!("Submitting project creation...");
            if let Some(team_id) = *selected_team {
                let project_name = project_name.to_string();
                let project_scope = if project_scope.is_empty() {
                    None
                } else {
                    Some(project_scope.to_string())
                };
                let error = error.clone();
                let navigator = navigator.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    log!("Creating project with name:", &project_name);
                    match ApiClient::get()
                        .create_project(project_name, project_scope, team_id)
                        .await
                    {
                        Ok(_) => {
                            log!("Project created successfully");
                            navigator.push(&ProjectRoute::Projects);
                        }
                        Err(e) => {
                            log!("Error creating project:", &e);
                            let error_message = format!("Ошибка при создании проекта: {}", e);
                            error.set(error_message);
                        }
                    }
                });
            } else {
                error.set("Выберите команду".to_string());
            }
        })
    };

    let on_cancel = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&ProjectRoute::Projects))
    };

    html! {
        <div class="create-project-page">
            <div class="create-project-form">
                <h2>{"Создание проекта"}</h2>
                if !error.is_empty() {
                    <div class="error-message">{error.to_string()}</div>
                }
                <div class="form-group">
                    <label for="team">{"Команда"}</label>
                    <select id="team" onchange={on_team_change}>
                        <option value="">{"Выберите команду"}</option>
                        {for teams.iter().map(|team| {
                            html! {
                                <option value={team.id.to_string()}>{&team.name}</option>
                            }
                        })}
                    </select>
                </div>
                <div class="form-group">
                    <label for="name">{"Название проекта"}</label>
                    <input
                        type="text"
                        id="name"
                        value={project_name.to_string()}
                        oninput={on_name_change}
                        placeholder="Введите название проекта"
                        required=true
                    />
                </div>
                <div class="form-group">
                    <label for="scope">{"Область действия"}</label>
                    <input
                        type="text"
                        id="scope"
                        value={project_scope.to_string()}
                        oninput={on_scope_change}
                        placeholder="Например: *.site.org"
                    />
                </div>
                <div class="form-buttons">
                    <button onclick={on_cancel} class="button secondary">{"Отмена"}</button>
                    <button onclick={on_submit} class="button">{"Создать"}</button>
                </div>
            </div>
        </div>
    }
} 