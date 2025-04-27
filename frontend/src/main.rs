use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::{MainRoute, switch_main};
use crate::context::user_provider::UserProvider;

mod components;
mod pages;
mod routes;
mod context;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <UserProvider>
                <Switch<MainRoute> render={switch_main} />
            </UserProvider>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}