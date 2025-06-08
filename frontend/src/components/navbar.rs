use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::main::MainRoute;
use crate::context::user_context::UserContext;
use crate::context::user_context::User;
use crate::api::ApiClient;
use gloo::console::log;
use crate::routes::admin::AdminRoute;


#[derive(Properties, PartialEq)]
pub struct NavbarProps {
    #[prop_or_default]
    pub project_title: Option<String>,
}

#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    let navigator = use_navigator().unwrap();
    let user_context = use_context::<UserContext>().unwrap();
    let user = user_context.clone();

    let on_logout = {
        let navigator = navigator.clone();
        let user_context = user_context.clone();
        Callback::from(move |_| {
            let navigator = navigator.clone();
            let user_context = user_context.clone();
            wasm_bindgen_futures::spawn_local(async move {
                log!("Logging out...");
                match ApiClient::get().logout().await {
                    Ok(_) => {
                        log!("Logged out successfully");
                        user_context.dispatch(User::default());
                        navigator.push(&MainRoute::Login);
                    }
                    Err(e) => {
                        log!("Error during logout:", &e);
                    }
                }
            });
        })
    };

    html! {
        <nav class="navbar">
            <div class="navbar-brand">
                <img src="/static/icons/logo.svg" class="icon" alt="Dark Molecule" />
            </div>
            if let Some(title) = &props.project_title {
                <div class="project-title">{title}</div>
            }
            if !user.is_all_none() {
                <div class="navbar-menu">
                    <ul>
                        // <li>
                        //     <Link<MainRoute> to={MainRoute::MainPage} classes="nav-link">
                        //         <span>{"Главная"}</span>
                        //     </Link<MainRoute>>
                        // </li>
                        <li>
                            <Link<MainRoute> to={MainRoute::Projects} classes="nav-link">
                                <span>{"Проекты"}</span>
                            </Link<MainRoute>>
                        </li>
                        {if let Some(is_admin) = user.is_admin {
                            if is_admin {
                                html! {
                                    <li>
                                        <Link<MainRoute> to={MainRoute::AdminRoot} classes="nav-link">
                                        <span>{"Администрирование"}</span>
                                    </Link<MainRoute>>
                                </li>
                                }
                            } else {
                                html! {}
                            }
                        } else {
                            html! {}
                        }}
                    </ul>
                </div>
            }
            <div class="navbar-right">
                if !user.is_all_none() {
                    <div class="user-info">
                        <Link<MainRoute> to={MainRoute::Profile} classes="profile-link">
                            if let Some(avatar) = &user.avatar {
                                <img src={avatar.clone()} class="avatar" alt="avatar" />
                            } else {
                                <div class="avatar-default">
                                    <img src="/static/icons/avatar.svg" class="icon" alt="avatar" />
                                </div>
                            }
                            <span class="username">{user.username.clone()}</span>
                        </Link<MainRoute>>
                        <button class="logout-button" onclick={on_logout}>
                            <img src="/static/icons/logout.svg" class="icon" alt="Выйти" />
                            <span>{"Выйти"}</span>
                        </button>
                    </div>
                } else {
                    <Link<MainRoute> to={MainRoute::Login} classes="nav-link">
                        <button class="button secondary">
                            <img src="/static/icons/login.svg" class="icon" alt="Войти" />
                            <span>{"Войти"}</span>
                        </button>
                    </Link<MainRoute>>
                }
            </div>
        </nav>
    }
}