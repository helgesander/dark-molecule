use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::footer::Footer;

mod routes;
mod components;
mod pages;

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<routes::MainRoute> render={routes::switch_main} />
            <Footer />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}