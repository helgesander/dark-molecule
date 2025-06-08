use yew::prelude::*;
use crate::api::ApiClient;
use crate::context::user_context::UserContext;

#[derive(Properties, PartialEq)]
pub struct CreateTeamModalProps {
    pub on_close: Callback<()>,
    pub on_team_created: Callback<()>,
    pub on_refresh_teams: Callback<()>,
}

#[function_component(CreateTeamModal)]
pub fn create_team_modal(props: &CreateTeamModalProps) -> Html {
    let name = use_state(String::new);
    let description = use_state(String::new);
    let error = use_state(String::new);
    let loading = use_state(|| false);
    let user = use_context::<UserContext>().unwrap();
    let on_name_change = {
        let name = name.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            name.set(input.value());
        })
    };

    let on_description_change = {
        let description = description.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            description.set(input.value());
        })
    };

    let on_submit = {
        let name = name.clone();
        let description = description.clone();
        let error = error.clone();
        let loading = loading.clone();
        let on_close = props.on_close.clone();
        let on_team_created = props.on_team_created.clone();
        let on_refresh_teams = props.on_refresh_teams.clone();
        let user_id = user.id;

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let name = name.to_string();
            let description = if description.is_empty() {
                None
            } else {
                Some(description.to_string())
            };
            let loading = loading.clone();
            let error = error.clone();
            let on_close = on_close.clone();
            let on_team_created = on_team_created.clone();
            let on_refresh_teams = on_refresh_teams.clone();

            loading.set(true);
            error.set(String::new());

            wasm_bindgen_futures::spawn_local(async move {
                if let Some(user_id) = user_id {
                    match ApiClient::get().create_team(name, description, user_id).await {
                        Ok(_) => {
                            on_team_created.emit(());
                            on_refresh_teams.emit(());
                            on_close.emit(());
                        }
                        Err(e) => {
                            error.set(e);
                            loading.set(false);
                        }
                    }
                } else {
                    error.set("User ID not found".to_string());
                    loading.set(false);
                }   
            });
        })
    };

    let on_close = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| {
            on_close.emit(());
        })
    };

    html! {
        <div class="modal-overlay">
            <div class="modal-content">
                <div class="modal-header">
                    <h2>{"Create New Team"}</h2>
                    <button class="close-button" onclick={on_close.clone()}>{"×"}</button>
                </div>
                <form onsubmit={on_submit}>
                    <div class="form-group">
                        <label for="team-name">{"Team Name"}</label>
                        <input
                            type="text"
                            id="team-name"
                            value={name.to_string()}
                            oninput={on_name_change}
                            required=true
                        />
                    </div>
                    <div class="form-group">
                        <label for="team-description">{"Description"}</label>
                        <textarea
                            id="team-description"
                            value={description.to_string()}
                            oninput={on_description_change}
                        />
                    </div>
                    if !error.is_empty() {
                        <div class="error-message">{error.to_string()}</div>
                    }
                    <div class="form-actions">
                        <button type="button" class="button secondary" onclick={on_close}>
                            {"Cancel"}
                        </button>
                        <button type="submit" class="button primary" disabled={*loading}>
                            if *loading {
                                {"Creating..."}
                            } else {
                                {"Create Team"}
                            }
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
} 