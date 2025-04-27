use yew::prelude::*;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct LoginFormProps {
    pub on_submit: Callback<(String, String)>,
    pub error: Option<String>,
}

#[function_component(LoginForm)]
pub fn login_form(props: &LoginFormProps) -> Html {
    let email = use_state(|| String::new());
    let password = use_state(|| String::new());

    let on_email_change = {
        let email = email.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            email.set(input.value());
        })
    };

    let on_password_change = {
        let password = password.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let on_submit = {
        let email = email.clone();
        let password = password.clone();
        let on_submit = props.on_submit.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            on_submit.emit(((*email).clone(), (*password).clone()));
        })
    };

    html! {
        <div class="content">
            <div class="login-container">
                <h1>{"Вход"}</h1>
                <form onsubmit={on_submit}>
                    <div class="input-group">
                        <label for="email">{"Почта"}</label>
                        <input
                            type="text"
                            id="email"
                            value={(*email).clone()}
                            onchange={on_email_change}
                            required={true}
                        />
                    </div>
                    <div class="input-group">
                        <label for="password">{"Пароль"}</label>
                        <input
                            type="password"
                            id="password"
                            value={(*password).clone()}
                            onchange={on_password_change}
                            required={true}
                        />
                    </div>
                    if let Some(error) = &props.error {
                        <div class="error-message">{error}</div>
                    }
                    <button type="submit" class="button">{"Войти"}</button>
                </form>
            </div>
        </div>
    }
} 