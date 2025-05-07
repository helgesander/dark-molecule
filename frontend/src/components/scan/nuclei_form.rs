use yew::prelude::*;

#[function_component(NucleiForm)]
pub fn nuclei_form() -> Html {
    html! {
        <div>
            <form>
                <div>
                    <label for="target">{"Хост"}</label>
                    <input type="text" id="target" value="127.0.0.1" />
                </div>
                <button type="submit">{"Сканировать"}</button>
            </form>
        </div>
    }
}
