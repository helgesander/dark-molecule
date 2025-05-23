use yew::prelude::*;
use yew_router::prelude::*;
use crate::api::{ApiClient, ScanRequest};
use uuid::Uuid;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub project_id: Uuid,
    pub on_close: Callback<()>,
    pub scan_type: String
}

#[function_component(ScanModal)]
pub fn scan_modal(props: &Props) -> Html {
    let loading = use_state(|| false);
    let target = use_state(|| String::new());
    let error = use_state(|| None::<String>);

    // Клонируем всё необходимое из props
    let project_id = props.project_id;
    let scan_type = props.scan_type.clone();
    let on_close = props.on_close.clone();

    let on_target_change = {
        let target = target.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            target.set(input.value());
        })
    };

    let on_submit = {
        let target = target.clone();
        let loading = loading.clone();
        let error = error.clone();
        let project_id = project_id;
        let scan_type = scan_type.clone();
        let on_close = on_close.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let target = target.to_string();
            let loading = loading.clone();
            let on_close = on_close.clone();
            let scan_type = scan_type.clone();
            let error = error.clone();

            loading.set(true);
            error.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                let request = ScanRequest {
                    r#type: scan_type,
                    target,
                };

                match ApiClient::get().create_scan(project_id, &request).await {
                    Ok(_) => {
                        loading.set(false);
                        on_close.emit(());
                    }
                    Err(e) => {
                        loading.set(false);
                        error.set(Some(e.to_string()));
                    }
                }
            });
        })
    };

    let on_close_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |_| {
            on_close.emit(());
        })
    };

    let title = if props.scan_type == "nmap" {
        "Сканирование сети"
    } else {
        "Сканирование целей на наличие уязвимостей"
    };

    let placeholder = if props.scan_type == "nmap" {
        "Введите сеть (например, 192.168.1.0/24)"
    } else {
        "Введите цель (сеть, IP-адрес, доменное имя)"
    };

    html! {
        <div class="modal">
            <div class="modal-content">
                <div class="modal-header">
                    <h2>{title}</h2>
                    <button class="close-button" onclick={on_close_click.clone()}>{"×"}</button>
                </div>
                <form onsubmit={on_submit}>
                    <div class="form-group">
                        <label for="target">{
                            if props.scan_type == "nmap" {
                                "Сеть"
                            } else {
                                "Цель"
                            }
                        }</label>
                        <input
                            type="text"
                            id="target"
                            value={(*target).clone()}
                            oninput={on_target_change}
                            placeholder={placeholder}
                            required=true
                        />
                    </div>
                    {if let Some(err) = &*error {
                        html! { <div class="error-message">{err}</div> }
                    } else {
                        html! {}
                    }}
                    <div class="form-actions">
                        <button type="submit" class="button primary" disabled={*loading}>
                            {if *loading {
                                "Загрузка..."
                            } else {
                                "Начать сканирование"
                            }}
                        </button>
                        <button type="button" class="button" onclick={on_close_click}>
                            {"Отменить"}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}