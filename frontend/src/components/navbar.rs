use yew::prelude::*;

#[function_component]
pub fn Navbar() -> Html {
    html! {
        <nav class="navbar">
            <ul>
                <li><a href="/">{"Главная"}</a></li>
                <li><a href="/about">{"О нас"}</a></li>
                <li><a href="/contact">{"Контакты"}</a></li>
            </ul>
        </nav>
    }
}