use crate::api::ApiClient;
use crate::context::user_context::UserContext;
use crate::debug_log;
use yew::prelude::*;

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
    let user_context = use_context::<UserContext>().unwrap();
    let user = user_context.clone();
    let error = use_state(|| String::new());

    {
        let user_context = user_context.clone();
        let error = error.clone();
        let id = user.id.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match ApiClient::get().get_user(&id).await {
                        Ok(full_user) => {
                            debug_log!("ProfilePage: got full user data: {:?}", full_user);
                        }
                        Err(e) => {
                            debug_log!("ProfilePage: error fetching user data: {}", &e);
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
        <div class="profile-page">
            <div class="profile-container">
                <div class="profile-header">
                    <div class="profile-avatar">
                        if let Some(avatar) = &user.avatar {
                            <img src={avatar.clone()} alt="Profile avatar" />
                        } else {
                            <div class="avatar-default">
                                <img src="/static/icons/avatar.svg" alt="Default avatar" />
                            </div>
                        }
                    </div>  
                    <div class="profile-info">
                        <h1>{&user.username.clone().unwrap_or_default()}</h1>
                        <p class="email">{&user.email.clone().unwrap_or_default()}</p>
                    </div>
                </div>
                <div class="profile-details">
                    <div class="detail-group">
                        <h3>{"Информация о пользователе "}</h3>
                        <div class="detail-item">
                            <span class="label">{"Роль:"}</span>
                            <span class="value">
                                {if user.is_admin.unwrap_or(false) { "Administrator" } else { "User" }}
                            </span>
                        </div>
                    </div>
                </div>
                if !error.is_empty() {
                    <div class="error-message">
                        {&*error}
                    </div>
                }
            </div>
        </div>
    }
} 