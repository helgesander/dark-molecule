use yew::prelude::*;

#[function_component(LoginPage)]
pub fn login_page() -> Html {
    let username = use_state(|| String::new());
    let password = use_state(|| String::new());
    let onclick = {
        let username = username.clone();
        let password = password.clone();

        Callback::from(move || {
            let username_value = (*username).clone();
            let password_value = (*password).clone(); 
        })
    };
    html! {
    <section class="main"   >
        <div class="login-container">
            <h1>{"Вход"}</h1>
            <div class="input-group">
                <label for="username">{"Имя пользователя"}</label>  
                <input type="text" id="username" placeholder="Введите имя пользователя" />
            </div>
            <div class="input-group">
                <label for="password">{"Пароль"}</label>
                <input type="password" id="password" placeholder="Введите пароль" />
            </div>
            <button class="button" {onclick}>{"Войти"}</button>
            <div class="signup-link">
                {"Нет аккаунта? "}
                <a href="/register">{"Зарегистрируйтесь"}</a>
            </div>
        </div>
    </section>
    }
}