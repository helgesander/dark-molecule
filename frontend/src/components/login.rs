use yew::prelude::*;
use yew_router::prelude::*;
use crate::context::user_context::{use_user_context, UserContext};
use crate::routes::MainRoute;
use gloo::console::log;
use gloo_net::http::Request;
use web_sys::RequestCredentials;

#[function_component(Login)]
pub fn login() -> Html {
    let user_context = use_user_context();
    let navigator = use_navigator().unwrap();
    let username = use_state(|| String::new());
    let password = use_state(|| String::new());
    let error = use_state(|| String::new());

    let on_username_change = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    let on_password_change = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let on_submit = {
        let username = username.clone();
        let password = password.clone();
        let error = error.clone();
        let user_context = user_context.clone();
        let navigator = navigator.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username = username.to_string();
            let password = password.to_string();
            let user_context = user_context.clone();
            let navigator = navigator.clone();
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::post("http://localhost:8000/api/auth/login")
                    .credentials(RequestCredentials::Include)
                    .json(&serde_json::json!({
                        "username": username,
                        "password": password
                    }))
                    .unwrap()
                    .send()
                    .await;

                match response {
                    Ok(resp) => {
                        if resp.ok() {
                            let user = resp.json::<crate::context::user_context::User>().await.unwrap();
                            user_context.dispatch(Some(user));
                            navigator.push(&MainRoute::MainPage);
                        } else {
                            error.set("Неверный логин или пароль".to_string());
                        }
                    }
                    Err(e) => {
                        log!("Error during login:", format!("{:?}", e));
                        error.set("Ошибка при входе".to_string());
                    }
                }
            });
        })
    };

    html! {
        <div class="login-container">
            <form class="login-form" onsubmit={on_submit}>
                <h2>{"Вход"}</h2>
                if !error.is_empty() {
                    <div class="error-message">{error.to_string()}</div>
                }
                <div class="form-group">
                    <label for="username">{"Логин"}</label>
                    <input
                        type="text"
                        id="username"
                        value={username.to_string()}
                        oninput={on_username_change}
                        required
                    />
                </div>
                <div class="form-group">
                    <label for="password">{"Пароль"}</label>
                    <input
                        type="password"
                        id="password"
                        value={password.to_string()}
                        oninput={on_password_change}
                        required
                    />
                </div>
                <button type="submit" class="login-button">{"Войти"}</button>
            </form>
        </div>
    }
} 