use yew::prelude::*;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html! {
        <div class="not-found-container">
            <h1>{"404"}</h1>
            <p>{"Эта страница не существует."}</p>
            <a href="/">{"Вернуться на главную"}</a>
        </div>
    }
}