use yew::prelude::*;
use yew_router::prelude::*;
use crate::routes::main::{MainRoute, switch_main};
use wasm_logger;
use crate::context::user_provider::UserProvider;
use crate::components::{
    navbar::Navbar,
    footer::Footer
};

mod routes;
mod pages;
mod components;
mod api;
mod context;

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <UserProvider>
                <div class="app-container">
                    <Navbar />
                    <Switch<MainRoute> render={switch_main} />
                    // TODO: maybe add footer here later
                </div>
            </UserProvider>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}