use yew::prelude::*;
use crate::api::ApiClient;
use crate::debug_log;
use crate::components::navbar::Navbar;
use crate::components::footer::Footer;
use crate::components::create_team_modal::CreateTeamModal;


#[derive(Properties, PartialEq)]
pub struct AdminUsersProps {
    pub size: usize,
    pub page: usize,
}

#[function_component(AdminUsers)]
pub fn admin_users() -> Html {
    let users = use_state(Vec::new);
    let error = use_state(String::new);
    let show_create_team_modal = use_state(|| false);

    {
        let users = users.clone();
        let error = error.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match ApiClient::get().get_users(Some(10), None).await {
                        Ok(fetched_users) => {
                            users.set(fetched_users);
                        }
                        Err(e) => {
                            debug_log!("Error fetching users: {}", e);
                            error.set(e);
                        }
                    }
                });
                || ()
            },
            (),
        );
    }

    html! {
        <>
            <main class="main-content">
                <div class="admin-users-page">
                    <div class="section-header">
                        <h2>{"Users Management"}</h2>
                        
                    </div>
                    if !error.is_empty() {
                        <div class="error-message">{error.to_string()}</div>
                    }
                    <div class="users-list">
                        {for users.iter().map(|user| {
                            html! {
                                <div class="user-card">
                                    <div class="user-info">
                                        <h3>{&user.username}</h3>
                                        <p class="email">{&user.email}</p>
                                    </div>
                                    <div class="user-actions">
                                        <button class="button secondary">{"Edit"}</button>
                                        <button class="button danger">{"Delete"}</button>
                                    </div>
                                </div>
                            }
                        })}
                    </div>
                </div>
            </main>
        </>
    }
} 