use yew::prelude::*;
use gloo::console::log;
use crate::api::{ApiClient, Team, CreateProjectRequest};
use uuid::Uuid;
use yew_router::prelude::*;
use crate::routes::project::ProjectRoute;
use crate::debug_log;
use web_sys::HtmlSelectElement;
use chrono::NaiveDate;


#[function_component(CreateProjectPage)]
pub fn create_project_page() -> Html {
    let teams = use_state(|| Vec::<Team>::new());
    let selected_team = use_state(|| None::<Uuid>);
    let project_name = use_state(|| String::new());
    let project_scope = use_state(|| String::new());
    let start_date = use_state(|| String::new());
    let end_date = use_state(|| String::new());
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
            debug_log!("Team change event triggered");
            if let Some(select) = e.target_dyn_into::<HtmlSelectElement>() {
                let value = select.value();
                debug_log!("Selected value: {}", value);
                if !value.is_empty() {
                    if let Ok(id) = Uuid::parse_str(&value) {
                        debug_log!("Setting selected team to {}", id);
                        selected_team.set(Some(id));
                    } else {
                        debug_log!("Failed to parse UUID from value: {}", value);
                    }
                } else {
                    debug_log!("Selected team is empty");
                    selected_team.set(None);
                }
            } else {
                debug_log!("Failed to convert event target to HtmlSelectElement");
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

    let on_start_date_change = {
        let start_date = start_date.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
            start_date.set(input.value());
        })
    };

    let on_end_date_change = {
        let end_date = end_date.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
            end_date.set(input.value());
        })
    };

    let on_submit = {
        let project_name = project_name.clone();
        let project_scope = project_scope.clone();
        let selected_team = selected_team.clone();
        let start_date = start_date.clone();
        let end_date = end_date.clone();
        let error = error.clone();
        let navigator = navigator.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            debug_log!("Submitting project creation...");
            debug_log!("Selected team: {:?}", *selected_team);
            if let Some(team_id) = *selected_team {
                let project_name = project_name.to_string();
                let project_scope = if project_scope.is_empty() {
                    None
                } else {
                    Some(project_scope.to_string())
                };
                
                let start_date = if !start_date.is_empty() {
                    NaiveDate::parse_from_str(&start_date, "%Y-%m-%d").ok()
                } else {
                    None
                };
                
                let end_date = if !end_date.is_empty() {
                    NaiveDate::parse_from_str(&end_date, "%Y-%m-%d").ok()
                } else {
                    None
                };

                let error = error.clone();
                let navigator = navigator.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    debug_log!("Creating project with name: {}", project_name);
                    match ApiClient::get()
                        .create_project(&CreateProjectRequest {
                            name: project_name,
                            scope: project_scope,
                            team_id,
                            folder: "pentest".to_string(),
                            start_date,
                            end_date,
                        })
                        .await
                    {
                        Ok(project) => {
                            debug_log!("Project created successfully");
                            navigator.push(&ProjectRoute::Project { id: project.id });
                        }
                        Err(e) => {
                            debug_log!("Error creating project: {}", e);
                            let error_message = format!("Ошибка при создании проекта: {}", e);
                            error.set(error_message);
                        }
                    }
                });
            } else {
                debug_log!("No team selected");
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
                <form onsubmit={on_submit}>
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
                    <div class="form-group">
                        <label for="start_date">{"Дата начала"}</label>
                        <input
                            type="date"
                            id="start_date"
                            value={start_date.to_string()}
                            oninput={on_start_date_change}
                        />
                    </div>
                    <div class="form-group">
                        <label for="end_date">{"Дата окончания"}</label>
                        <input
                            type="date"
                            id="end_date"
                            value={end_date.to_string()}
                            oninput={on_end_date_change}
                        />
                    </div>
                    <div class="form-buttons">
                        <button type="button" onclick={on_cancel} class="button secondary">{"Отмена"}</button>
                        <button type="submit" class="button primary">{"Создать"}</button>
                    </div>
                </form>
            </div>
        </div>
    }
} 