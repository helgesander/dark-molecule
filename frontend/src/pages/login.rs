use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::main::MainRoute;
use crate::api::{ApiClient, User};
use crate::context::user_context::{from_api_to_context, UserContext};
use gloo::console::log;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let email = use_state(|| String::new());
    let password = use_state(|| String::new());
    let error = use_state(|| String::new());
    let navigator = use_navigator().unwrap();
    let user_context = use_context::<UserContext>().unwrap();

    {
        let user_context_clone = user_context.clone();
        use_effect_with_deps(move |_| {
            log!("LoginPage: current user context:", format!("{:?}", &user_context_clone));
            || {}
        }, ());
    }

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
                log!("LoginPage: attempting login with email:", &email);
                match ApiClient::get().login(email, password).await {
                    Ok(user) => {
                        log!("LoginPage: login successful, user:", format!("{:?}", user));
                        match ApiClient::get().get_user(&user.id).await {
                            Ok(full_user) => {
                                log!("LoginPage: got full user data:", format!("{:?}", full_user));
                                user_context.dispatch(from_api_to_context(full_user));
                            }
                            Err(_) => {
                                log!("LoginPage: using basic user data");
                                user_context.dispatch(from_api_to_context(user));
                            }
                        }
                        navigator.push(&MainRoute::Projects);
                    }
                    Err(e) => {
                        log!("LoginPage: login error:", &e);
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