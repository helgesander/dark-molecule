use yew::prelude::*;

#[function_component(RegisterPage)]
pub fn register_page() -> Html {
    html! {
    <section class="main" style="text-align: center">
        <div class="register-container">
            <h1>{"Регистрация"}</h1>
            <div class="input-group">
                <label for="username">{"Имя пользователя"}</label>
                <input type="text" id="username" placeholder="Введите имя пользователя" />
            </div>
            <div class="input-group">
                <label for="password">{"Пароль"}</label>
                <input type="password" id="password" placeholder="Введите пароль" />
            </div>
            <button class="button">{"Зарегистрироваться"}</button>
            <div class="login-link">
                {"Есть аккаунт? "}
                <a href="/login">{"Войти"}</a>
            </div>
        </div>
    </section>
    }
}