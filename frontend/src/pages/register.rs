use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::main::MainRoute;
use crate::api::ApiClient;

#[function_component(RegisterPage)]
pub fn register_page() -> Html {
    let username = use_state(|| String::new());
    let email = use_state(|| String::new());
    let password = use_state(|| String::new());
    let error = use_state(|| String::new());
    let navigator = use_navigator().unwrap();

    let on_username_change = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };
    let on_email_change = {
        let email = email.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
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
        let email = email.clone();
        let password = password.clone();
        let error = error.clone();
        let navigator = navigator.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username = username.to_string();
            let email = email.to_string();
            let password = password.to_string();
            let error = error.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                // Здесь должен быть вызов регистрации через ApiClient
                // Пример:
                match ApiClient::get().register(username, email, password).await {
                    Ok(_) => {
                        navigator.push(&MainRoute::Login);
                    }
                    Err(e) => {
                        error.set(e);
                    }
                }
            });
        })
    };

    html! {
        <div class="login-page">
            <form class="login-form" onsubmit={on_submit}>
                <h2>{"Регистрация"}</h2>
                if !error.is_empty() {
                    <div class="error-message">{error.to_string()}</div>
                }
                <div class="form-group">
                    <label for="username">{"Имя пользователя"}</label>
                    <input
                        type="text"
                        id="username"
                        value={username.to_string()}
                        oninput={on_username_change}
                        placeholder="Введите имя пользователя"
                        required=true
                    />
                </div>
                <div class="form-group">
                    <label for="email">{"Почта"}</label>
                    <input
                        type="email"
                        id="email"
                        value={email.to_string()}
                        oninput={on_email_change}
                        placeholder="you@example.com"
                        required=true
                    />
                </div>
                <div class="form-group">
                    <label for="password">{"Пароль"}</label>
                    <input
                        type="password"
                        id="password"
                        value={password.to_string()}
                        oninput={on_password_change}
                        placeholder="Введите пароль"
                        required=true
                    />
                </div>
                <button type="submit" class="login-button">{"Зарегистрироваться"}</button>
                <div class="login-link">
                    {"Есть аккаунт? "}
                    <Link<MainRoute> to={MainRoute::Login}>{"Войти"}</Link<MainRoute>>
                </div>
            </form>
        </div>
    }
}