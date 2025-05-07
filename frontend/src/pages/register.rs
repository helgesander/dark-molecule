use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::main::MainRoute;
use crate::api::UserForm;
use crate::api::ApiClient;

#[function_component(RegisterPage)]
pub fn register_page() -> Html {
    let username = use_state(|| String::new());
    let email = use_state(|| String::new());
    let password = use_state(|| String::new());
    let first_name = use_state(|| String::new());
    let last_name = use_state(|| String::new());
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

    let on_first_name_change = {
        let first_name = first_name.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            first_name.set(input.value());
        })
    };

    let on_last_name_change = {
        let last_name = last_name.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            last_name.set(input.value());
        })
    };

    let on_submit = {
        let username = username.clone();
        let email = email.clone();
        let password = password.clone();
        let first_name = first_name.clone();
        let last_name = last_name.clone();
        let error = error.clone();
        let navigator = navigator.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let username = username.to_string();
            let email = email.to_string();
            let password = password.to_string();
            let first_name = first_name.to_string();
            let last_name = last_name.to_string();
            let error = error.clone();
            let navigator = navigator.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match ApiClient::get().register(&UserForm {
                    username,
                    email,
                    password,
                    first_name: if first_name.is_empty() { None } else { Some(first_name) },
                    last_name: if last_name.is_empty() { None } else { Some(last_name) },
                }).await {
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
                    <label for="first_name">{"Имя"}</label>
                    <input
                        type="text"
                        id="first_name"
                        value={first_name.to_string()}
                        oninput={on_first_name_change}
                        placeholder="Иван"
                    />
                </div>
                <div class="form-group">
                    <label for="last_name">{"Фамилия"}</label>
                    <input
                        type="text"
                        id="last_name"
                        value={last_name.to_string()}
                        oninput={on_last_name_change}
                        placeholder="Иванов"
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
