use yew::prelude::*;
use yew_router::prelude::*;
use crate::context::user_context::{use_user_context, UserContext};
use crate::routes::MainRoute;
use gloo::console::log;
use gloo_net::http::Request;
use web_sys::RequestCredentials;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let user_context = use_user_context();
    let user = user_context.user.clone();

    let handle_logout = {
        let user_context = user_context.clone();
        Callback::from(move |_| {
            user_context.dispatch(None);
        })
    };

    html! {
        <nav class="navbar">
            <div class="navbar-brand">
                <Link<MainRoute> to={MainRoute::MainPage}>{"Dark Molecule"}</Link<MainRoute>>
            </div>
            <ul>
                <li>
                    <Link<MainRoute> to={MainRoute::MainPage}>{"Главная"}</Link<MainRoute>>
                </li>
                <li>
                    <Link<MainRoute> to={MainRoute::Projects}>{"Проекты"}</Link<MainRoute>>
                </li>
            </ul>
            <div class="navbar-right">
                if let Some(user) = user {
                    <div class="user-info">
                        <span class="username">{user.username}</span>
                        <button class="logout-button" onclick={handle_logout}>{"Выйти"}</button>
                    </div>
                } else {
                    <Link<MainRoute> to={MainRoute::Login}>{"Войти"}</Link<MainRoute>>
                }
            </div>
        </nav>
    }
}