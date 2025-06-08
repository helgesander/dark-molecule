use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IssueCreateFormProps {
    pub on_submit: Callback<String>,  
    pub on_cancel: Callback<()>,      
}

#[function_component(IssueCreateForm)]
pub fn issue_create_form(props: &IssueCreateFormProps) -> Html {
    let issue_name = use_state(|| String::new());

    let on_name_change = {
        let issue_name = issue_name.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                issue_name.set(input.value());
            }
        })
    };

    let on_form_submit = {
        let issue_name = issue_name.clone();
        let on_submit = props.on_submit.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            on_submit.emit((*issue_name).clone());
            issue_name.set(String::new()); 
        })
    };

    html! {
        <div class="modal">
            <div class="modal-content">
                <div class="modal-header">
                    <h2>{"Создать новую уязвимость"}</h2>
                    <button class="close-button">{"×"}</button>
                </div>
                    <form onsubmit={on_form_submit}>
                        <div class="form-group">
                            <label>
                                {"Название уязвимости:"}
                                <input
                                    type="text"
                                    value={(*issue_name).clone()}
                                    oninput={on_name_change}
                                    required=true
                                    placeholder="Введите название уязвимости"
                                />
                            </label>
                        </div>
                        <div class="form-actions">
                            <button type="submit" class="btn btn-primary">{"Создать"}</button>
                            <button
                                type="button"
                                onclick={props.on_cancel.reform(|_| ())}
                                class="btn btn-secondary"
                            >
                                {"Отмена"}
                            </button>
                        </div>
                </form>
            </div>
        </div>
    }
}