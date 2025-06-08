use yew::prelude::*;
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
    let targets = use_state(|| vec![String::new()]);
    let loading = use_state(|| false);
    let error = use_state(|| None::<String>);

    let project_id = props.project_id;
    let scan_type = props.scan_type.clone();
    let on_close = props.on_close.clone();

    let on_target_change = {
        let targets = targets.clone();
        Callback::from(move |(index, e): (usize, InputEvent)| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let mut new_targets = (*targets).clone();
            new_targets[index] = input.value();
            targets.set(new_targets);
        })
    };

    let add_target = {
        let targets = targets.clone();
        Callback::from(move |_| {
            let mut new_targets = (*targets).clone();
            new_targets.push(String::new());
            targets.set(new_targets);
        })
    };

    let remove_target = {
        let targets = targets.clone();
        Callback::from(move |index: usize| {
            let mut new_targets = (*targets).clone();
            if new_targets.len() > 1 {
                new_targets.remove(index);
                targets.set(new_targets);
            }
        })
    };

    let on_submit = {
        let targets = targets.clone();
        let loading = loading.clone();
        let error = error.clone();
        let project_id = project_id;
        let scan_type = scan_type.clone();
        let on_close = on_close.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let targets = (*targets).clone();
            let loading = loading.clone();
            let on_close = on_close.clone();
            let scan_type = scan_type.clone();
            let error = error.clone();

            // Filter out empty targets
            let valid_targets: Vec<String> = targets.into_iter()
                .filter(|t| !t.trim().is_empty())
                .collect();

            if valid_targets.is_empty() {
                error.set(Some("Пожалуйста, укажите хотя бы одну цель".to_string()));
                return;
            }

            loading.set(true);
            error.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                let request = ScanRequest {
                    r#type: scan_type,
                    target: valid_targets.join(" "),
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
                        <label>{
                            if props.scan_type == "nmap" {
                                "Сети"
                            } else {
                                "Цели"
                            }
                        }</label>
                        {(*targets).iter().enumerate().map(|(index, target)| {
                            html! {
                                <div class="target-input-group">
                                    <input
                                        type="text"
                                        value={target.clone()}
                                        oninput={on_target_change.reform(move |e| (index, e))}
                                        placeholder={placeholder}
                                        required=true
                                    />
                                    if (*targets).len() > 1 {
                                        <button
                                            type="button"
                                            class="remove-target"
                                            onclick={remove_target.reform(move |_| index)}
                                        >
                                            {"×"}
                                        </button>
                                    }
                                </div>
                            }
                        }).collect::<Html>()}
                        <button
                            type="button"
                            class="add-target"
                            onclick={add_target}
                        >
                            {"+ Добавить цель"}
                        </button>
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