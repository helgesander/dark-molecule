use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::MainRoute;
use gloo::console::log;
use gloo_net::http::Request;
use web_sys::RequestCredentials;
use crate::context::user_context::use_user_context;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let navigator = use_navigator().unwrap();
    let current_route = use_route::<MainRoute>().unwrap_or(MainRoute::MainPage);
    let user_context = use_user_context();

    log!("Current route:", format!("{:?}", current_route));

    let on_logout = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            log!("Logout button clicked");
            
            // Отправляем запрос на выход
            let navigator = navigator.clone();
            let user_context = user_context.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::post("http://localhost:8000/api/auth/logout")
                    .credentials(RequestCredentials::Include)
                    .send()
                    .await;

                match response {
                    Ok(_) => {
                        user_context.set_user.emit(None);
                        navigator.push(&MainRoute::Login);
                    }
                    Err(e) => {
                        log!("Error during logout:", format!("{:?}", e));
                        navigator.push(&MainRoute::Login);
                    }
                }
            });
        })
    };

    html! {
        <nav class="navbar">
            <Link<MainRoute> to={MainRoute::MainPage} classes="navbar-brand">
                {"Dark Molecule"}
            </Link<MainRoute>>

            <ul>
                <li>
                    <Link<MainRoute> 
                        to={MainRoute::Projects} 
                        classes={classes!(if current_route == MainRoute::Projects { "active" } else { "" })}
                    >
                        {"Проекты"}
                    </Link<MainRoute>>
                </li>
                <li>
                    <Link<MainRoute> 
                        to={MainRoute::Admin} 
                        classes={classes!(if matches!(current_route, MainRoute::Admin | MainRoute::AdminRoot) { "active" } else { "" })}
                    >
                        {"Админка"}
                    </Link<MainRoute>>
                </li>
            </ul>

            <div class="navbar-right">
                <div class="user-info">
                    <span class="username">
                        {user_context.user.as_ref().map(|u| &u.username).unwrap_or("Гость")}
                    </span>
                    if user_context.user.is_some() {
                        <button class="logout-button" onclick={on_logout}>
                            {"Выйти"}
                        </button>
                    }
                </div>
            </div>
        </nav>
    }
}