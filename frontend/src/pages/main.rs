use yew::prelude::*;

use crate::components::navbar::Navbar;

#[function_component(MainPage)]
pub fn main_page() -> Html {
    html! {
        <div class="container">
            <Navbar />
        </div>
    }
}