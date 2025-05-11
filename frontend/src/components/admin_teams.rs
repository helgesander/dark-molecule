use yew::prelude::*;
use crate::api::Team;
use crate::components::create_team_modal::CreateTeamModal;
use crate::api::ApiClient;


#[derive(Properties, PartialEq)]
pub struct AdminTeamsProps {
    pub teams: Vec<Team>,
}

#[function_component(AdminTeams)]
pub fn admin_teams() -> Html {
    let teams = use_state(|| Vec::<Team>::new());
    let error = use_state(|| String::new());
    let show_create_team_modal = use_state(|| false);

    let refresh_teams = {
        let teams = teams.clone();
        let error = error.clone();
        Callback::from(move |_| {
            let teams = teams.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match ApiClient::get().get_teams().await {
                    Ok(new_teams) => teams.set(new_teams),
                    Err(e) => error.set(e.to_string()),
                }
            });
        })
    };

    {
        let teams = teams.clone();
        let error = error.clone();
        
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match ApiClient::get().get_teams().await {
                        Ok(new_teams) => teams.set(new_teams),
                        Err(e) => error.set(e.to_string()),
                    }
                });
                || ()
            },
            (),
        );
    }

    let on_create_team_click = {
        let show_create_team_modal = show_create_team_modal.clone();
        Callback::from(move |_| {
            show_create_team_modal.set(true);
        })
    };

    let on_modal_close = {
        let show_create_team_modal = show_create_team_modal.clone();
        Callback::from(move |_| {
            show_create_team_modal.set(false);
        })
    };

    let on_team_created = {
        let show_create_team_modal = show_create_team_modal.clone();
        Callback::from(move |_| {
            show_create_team_modal.set(false);
        })
    };
    
    html! {
        <>
            <main class="main-content">
                <div class="admin-teams-page">
                    <div class="section-header">
                        <h2>{"Teams"}</h2>
                    </div>
                    <div class="header-actions">
                        <button class="button primary" onclick={on_create_team_click}>
                            {"Create Team"}
                        </button>
                    </div>
                    if !error.is_empty() {
                        <div class="error-message">{error.to_string()}</div>
                    }
                    <div class="teams-list">
                        {for teams.iter().map(|team| {
                            html! {
                                <div class="team-card">
                                    <h3>{&team.name}</h3>
                                </div>
                            }
                        })}
                    </div>
                </div>
            </main>
            if *show_create_team_modal {
                <CreateTeamModal 
                    on_close={on_modal_close}
                    on_team_created={on_team_created}
                    on_refresh_teams={refresh_teams}
                />
            }
        </>
    }
}