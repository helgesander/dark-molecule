use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::main::MainRoute;
use crate::api::{ApiClient, User};
use crate::context::user_context::{use_user_context, User as ContextUser};

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let email = use_state(|| String::new());
    let password = use_state(|| String::new());
    let error = use_state(|| String::new());
    let navigator = use_navigator().unwrap();
    let user_context = use_user_context();

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

    let to_context_user = |u: User| ContextUser {
        username: u.username,
        email: u.email,
        is_admin: u.is_admin,
        avatar: u.avatar,
    };

    let on_submit = {
        let email = email.clone();
        let password = password.clone();
        let error = error.clone();
        let navigator = navigator.clone();
        let user_context = user_context.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let email = email.to_string();
            let password = password.to_string();
            let error = error.clone();
            let navigator = navigator.clone();
            let user_context = user_context.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match ApiClient::get().login(email, password).await {
                    Ok(user) => {
                        let user_id = user.id.to_string();
                        match ApiClient::get().get_user(&user_id).await {
                            Ok(full_user) => {
                                user_context.dispatch(Some(to_context_user(full_user)));
                            }
                            Err(_) => {
                                user_context.dispatch(Some(to_context_user(user)));
                            }
                        }
                        navigator.push(&MainRoute::Projects);
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
                <h2>{"Вход в аккаунт"}</h2>
                if !error.is_empty() {
                    <div class="error-message">{error.to_string()}</div>
                }
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
                <button type="submit" class="login-button">{"Войти"}</button>
                <div class="login-link">
                    {"Нет аккаунта? "}
                    <Link<MainRoute> to={MainRoute::Register}>{"Зарегистрироваться"}</Link<MainRoute>>
                </div>
            </form>
        </div>
    }
}