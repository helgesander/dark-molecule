use yew::prelude::*;
use yew_router::prelude::*;
use crate::context::user_context::{use_user_context, UserContext};
use crate::routes::main::MainRoute;


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
                <img src="/static/icons/logo.svg" class="icon" alt="Dark Molecule" />
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
                        if let Some(avatar) = &user.avatar {
                            <img src={avatar.clone()} class="avatar" alt="avatar" />
                        } else {
                            <div class="avatar-default">{user.username.chars().next().unwrap_or('?')}</div>
                        }
                        <span class="username">{user.username.clone()}</span>
                        <button class="logout-button" onclick={handle_logout}>{"Выйти"}</button>
                    </div>
                } else {
                    <Link<MainRoute> to={MainRoute::Login}>{"Войти"}</Link<MainRoute>>
                }
            </div>
        </nav>
    }
}