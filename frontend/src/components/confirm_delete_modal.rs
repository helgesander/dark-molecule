use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_confirm: Callback<()>,
    pub on_cancel: Callback<()>,
    pub title: String,
    pub message: String,
}

#[function_component(ConfirmDeleteModal)]
pub fn confirm_delete_modal(props: &Props) -> Html {
    let loading = use_state(|| false);
    let error = use_state(|| None::<String>);

    let on_confirm = {
        let on_confirm = props.on_confirm.clone();
        let loading = loading.clone();
        let error = error.clone();
        Callback::from(move |_| {
            loading.set(true);
            error.set(None);
            on_confirm.emit(());
        })
    };

    let on_cancel_click = {
        let on_cancel = props.on_cancel.clone();
        Callback::from(move |_| on_cancel.emit(()))
    };

    html! {
        <div class="modal">
            <div class="modal-content">
                <div class="modal-header">
                    <h2>{props.title.clone()}</h2>
                    <button class="close-button" onclick={on_cancel_click.clone()}>{"×"}</button>
                </div>
                <div class="modal-body">
                    <p>{props.message.clone()}</p>
                </div>
                <div class="form-actions">
                    <button class="button secondary" onclick={on_cancel_click}>{"Отмена"}</button>
                    <button class="button primary" onclick={on_confirm}>{"Удалить"}</button>
                </div>
            </div>
        </div>
    }
} 