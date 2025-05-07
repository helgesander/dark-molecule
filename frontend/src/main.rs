use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::main::{MainRoute, switch_main};
use wasm_logger;
use gloo::console::log;
use crate::components::navbar::Navbar;
use crate::context::user_context::{UserContext, create_user_context};
use crate::context::user_provider::UserProvider;


mod routes;
mod pages;
mod components;
mod api;
mod context;
mod utils;

#[function_component(App)]
fn app() -> Html {
    html! {
        <UserProvider>
            <BrowserRouter>
                <Navbar />
                <Switch<MainRoute> render={switch_main} />
            </BrowserRouter>
        </UserProvider>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}