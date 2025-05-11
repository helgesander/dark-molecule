use yew::prelude::*;
use crate::api::ApiClient;
use crate::debug_log;
use crate::api::UserForm;
use crate::components::navbar::Navbar;
use crate::components::footer::Footer;

#[function_component(AdminCreateUserPage)]
pub fn admin_create_user_page() -> Html {
    let username = use_state(|| String::new());
    let email = use_state(|| String::new());
    let password = use_state(|| String::new());
    let is_admin = use_state(|| false);
    let error = use_state(|| String::new());

    let on_submit = {
        let username = username.clone();
        let email = email.clone();
        let password = password.clone();
        let is_admin = is_admin.clone();
        let error = error.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username = (*username).clone();
            let email = (*email).clone();
            let password = (*password).clone();
            let is_admin = *is_admin;
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match ApiClient::get().create_user(&UserForm { first_name: None, last_name: None, username, email, password, is_admin: Some(is_admin) }).await {
                    Ok(_) => {
                        debug_log!("User created successfully");
                        // TODO: Redirect to users list or show success message
                    }
                    Err(e) => {
                        debug_log!("Error creating user: {}", e);
                        error.set(e);
                    }
                }
            });
        })
    };

    html! {
        <>
            <Navbar />
            <main class="main-content">
                <div class="create-user-page">
                    <div class="section-header">
                        <h2>{"Create New User"}</h2>
                    </div>
                    if !error.is_empty() {
                        <div class="error-message">{error.to_string()}</div>
                    }
                    <form onsubmit={on_submit} class="create-user-form">
                        <div class="form-group">
                            <label for="username">{"Username"}</label>
                            <input 
                                type="text" 
                                id="username"
                                value={(*username).clone()}
                                onchange={let username = username.clone(); Callback::from(move |e: Event| {
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    username.set(input.value());
                                })}
                                required={true}
                            />
                        </div>
                        <div class="form-group">
                            <label for="email">{"Email"}</label>
                            <input 
                                type="email" 
                                id="email"
                                value={(*email).clone()}
                                onchange={let email = email.clone(); Callback::from(move |e: Event| {
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    email.set(input.value());
                                })}
                                required={true}
                            />
                        </div>
                        <div class="form-group">
                            <label for="password">{"Password"}</label>
                            <input 
                                type="password" 
                                id="password"
                                value={(*password).clone()}
                                onchange={let password = password.clone(); Callback::from(move |e: Event| {
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    password.set(input.value());
                                })}
                                required={true}
                            />
                        </div>
                        <div class="form-group checkbox">
                            <label>
                                <input 
                                    type="checkbox"
                                    checked={*is_admin}
                                    onchange={let is_admin = is_admin.clone(); Callback::from(move |e: Event| {
                                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                        is_admin.set(input.checked());
                                    })}
                                />
                                {"Administrator"}
                            </label>
                        </div>
                        <div class="form-actions">
                            <button type="submit" class="button primary">{"Create User"}</button>
                        </div>
                    </form>
                </div>
            </main>
            <Footer />
        </>
    }
} 