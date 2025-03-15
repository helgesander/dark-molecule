use yew::prelude::*;
use yew_router::prelude::*;

mod routes;

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<routes::MainRoute> render={routes::switch_main} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}