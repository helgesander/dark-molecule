use yew::prelude::*;
use yew_router::prelude::*;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;
use crate::components::login_form::LoginForm;
use crate::routes::MainRoute;
use web_sys::RequestCredentials;
use crate::context::user_context::{use_user_context, User};

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginResponse {
    user: User,
}

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let error = use_state(|| None);
    let navigator = use_navigator().unwrap();
    let user_context = use_user_context();

    let on_submit = {
        let error = error.clone();
        let navigator = navigator.clone();
        let user_context = user_context.clone();
        
        Callback::from(move |(email, password): (String, String)| {
            let error = error.clone();
            let navigator = navigator.clone();
            let user_context = user_context.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let login_request = LoginRequest {
                    email,
                    password,
                };

                let response = Request::post("http://localhost:8000/api/auth/")
                    .header("Content-Type", "application/json")
                    .credentials(RequestCredentials::Include)
                    .json(&login_request)
                    .unwrap()
                    .send()
                    .await;

                match response {
                    Ok(resp) => {
                        if resp.ok() {
                            if let Ok(login_response) = resp.json::<LoginResponse>().await {
                                user_context.set_user.emit(Some(login_response.user));
                                navigator.push(&MainRoute::Projects);
                            }
                        } else {
                            error.set(Some("Неверные учетные данные".to_string()));
                        }
                    }
                    Err(e) => {
                        error.set(Some(format!("Ошибка: {}", e)));
                    }
                }
            });
        })
    };

    html! {
        <LoginForm on_submit={on_submit} error={(*error).clone()} />
    }
}