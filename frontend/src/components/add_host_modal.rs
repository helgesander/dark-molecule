use yew::prelude::*;
use crate::api::{ApiClient, CreateHostRequest, Host};
use crate::debug_log;
use uuid::Uuid;
use web_sys::HtmlInputElement;

#[derive(Properties, PartialEq)]
pub struct AddHostModalProps {
    pub project_id: Uuid,
    pub on_close: Callback<()>,
    pub header: String,
    pub host: Option<Host>,
}

#[function_component(AddHostModal)]
pub fn add_host_modal(props: &AddHostModalProps) -> Html {
    let hostname = use_state(|| props.host.as_ref().and_then(|h| h.hostname.clone()).unwrap_or_default());
    let ip = use_state(|| props.host.as_ref().map(|h| h.ip_address.clone()).unwrap_or_default());
    let show_success = use_state(|| false);
    let error_message = use_state(|| None::<String>);

    let on_hostname_change = {
        let hostname = hostname.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            hostname.set(input.value());
        })
    };

    let on_ip_change = {
        let ip = ip.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            ip.set(input.value());
        })
    };

    let on_submit = {
        let hostname = hostname.clone();
        let ip = ip.clone();
        let project_id = props.project_id;
        let on_close = props.on_close.clone();
        let show_success = show_success.clone();
        let error_message = error_message.clone();
        let is_edit = props.host.is_some();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            let new_host = CreateHostRequest {
                hostname: Some((*hostname).clone()),
                ip_address: (*ip).clone(),
            };

            let show_success = show_success.clone();
            let error_message = error_message.clone();
            let on_close = on_close.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                let result = if is_edit {
                    // TODO: Implement update_host in ApiClient
                    ApiClient::get().update_host(project_id, new_host).await
                } else {
                    ApiClient::get().create_host(project_id, new_host).await
                };

                match result {
                    Ok(_) => {
                        debug_log!("Host {} successfully", if is_edit { "updated" } else { "created" });
                        show_success.set(true);
                        let timeout = gloo::timers::callback::Timeout::new(2_000, move || {
                            on_close.emit(());
                        });
                        timeout.forget();
                    },
                    Err(e) => {
                        debug_log!("Failed to {} host: {}", if is_edit { "update" } else { "create" }, e);
                        error_message.set(Some(format!("Ошибка при {} хоста: {}", 
                            if is_edit { "обновлении" } else { "создании" }, e)));
                    }
                }
            });
        })
    };

    html! {
        <div class="modal-backdrop">
            <div class="modal-content">
                <div class="modal-header">
                    <h2>{&props.header}</h2>
                    <button class="close-button" onclick={let on_close = props.on_close.clone(); Callback::from(move |_| on_close.emit(()))}>
                        {"×"}
                    </button>
                </div>
                <form onsubmit={on_submit}>
                    <div class="form-group">
                        <label for="hostname">{"Имя хоста"}</label>
                        <input
                            type="text"
                            id="hostname"
                            class="form-control"
                            value={(*hostname).clone()}
                            onchange={on_hostname_change}
                            required=true
                        />
                    </div>
                    
                    <div class="form-group">
                        <label for="ip">{"IP адрес"}</label>
                        <input
                            type="text"
                            id="ip"
                            class="form-control"
                            value={(*ip).clone()}
                            onchange={on_ip_change}
                            required=true
                        />
                    </div>

                    if let Some(error) = (*error_message).clone() {
                        <div class="error-message">{error}</div>
                    }

                    if *show_success {
                        <div class="success-message">
                            {if props.host.is_some() { "Хост успешно обновлен" } else { "Хост успешно добавлен" }}
                        </div>
                    }

                    <div class="modal-footer">
                        <button type="button" class="btn btn-secondary" onclick={let on_close = props.on_close.clone(); Callback::from(move |_| on_close.emit(()))}>
                            {"Отмена"}
                        </button>
                        <button type="submit" class="btn btn-primary">
                            {if props.host.is_some() { "Сохранить" } else { "Добавить" }}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
} 