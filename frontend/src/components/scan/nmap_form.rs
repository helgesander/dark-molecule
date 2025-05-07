use yew::prelude::*;

#[function_component(NmapForm)]
pub fn nmap_form() -> Html {
    html! {
        <div class="nmap-form">
            <form>
                <div>
                    <label for="target">{"Сеть"}</label>
                    <input type="text" id="target" />
                </div>
                <button type="submit">{"Сканировать"}</button>
            </form>
        </div>
    }
}