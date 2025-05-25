use yew::prelude::*;
use web_sys::{HtmlInputElement, File, FormData};
use wasm_bindgen::JsCast;
use crate::api::ApiClient;

#[function_component(AdminReportTemplates)]
pub fn admin_report_templates() -> Html {
    let name = use_state(String::new);
    let description = use_state(String::new);
    let file = use_state(|| None::<File>);
    let success_message = use_state(|| None::<String>);
    let error_message = use_state(|| None::<String>);
    let templates = use_state(Vec::new);

    // Загрузка списка шаблонов при монтировании компонента
    {
        let templates = templates.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(template_list) = ApiClient::get().get_report_templates().await {
                        templates.set(template_list);
                    }
                });
                || ()
            },
            (),
        );
    }

    let onname = {
        let name = name.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            name.set(input.value());
        })
    };

    let ondescription = {
        let description = description.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            description.set(input.value());
        })
    };

    let onfile = {
        let file = file.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(file_list) = input.files() {
                if let Some(selected_file) = file_list.get(0) {
                    file.set(Some(selected_file));
                }
            }
        })
    };

    let onsubmit = {
        let name = name.clone();
        let description = description.clone();
        let file = file.clone();
        let success_message = success_message.clone();
        let error_message = error_message.clone();
        let templates = templates.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            if let Some(upload_file) = (*file).clone() {
                let form_data = FormData::new().unwrap();
                form_data.append_with_str("name", &(*name)).unwrap();
                form_data.append_with_str("description", &(*description)).unwrap();
                form_data.append_with_blob("file", &upload_file).unwrap();

                let name = name.clone();
                let description = description.clone();
                let file = file.clone();
                let success_message = success_message.clone();
                let error_message = error_message.clone();
                let templates = templates.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    match ApiClient::get().create_report_template(form_data).await {
                        Ok(_) => {
                            success_message.set(Some("Шаблон успешно загружен".to_string()));
                            error_message.set(None);
                            name.set(String::new());
                            description.set(String::new());
                            file.set(None);
                            
                            // Обновляем список шаблонов после успешной загрузки
                            if let Ok(template_list) = ApiClient::get().get_report_templates().await {
                                templates.set(template_list);
                            }
                        }
                        Err(e) => {
                            error_message.set(Some(e.to_string()));
                            success_message.set(None);
                        }
                    }
                });
            } else {
                error_message.set(Some("Пожалуйста, выберите файл шаблона".to_string()));
            }
        })
    };

    html! {
        <div class="admin-report-templates">
            <div class="templates-container">
                <div class="upload-form-column">
        <div class="template-upload-form">
            <h2>{"Загрузка шаблона отчета"}</h2>

            if let Some(message) = (*success_message).clone() {
                <div class="success-message">
                    {message}
                </div>
            }

            if let Some(error) = (*error_message).clone() {
                <div class="error-message">
                    {error}
                </div>
            }

            <form {onsubmit}>
                <div class="form-group">
                    <label for="name">{"Название шаблона"}</label>
                    <input
                        type="text"
                        id="name"
                        class="form-control"
                        value={(*name).clone()}
                        onchange={onname}
                        required=true
                        placeholder="Введите название шаблона"
                    />
                </div>

                <div class="form-group">
                    <label for="template-file">{"Файл шаблона"}</label>
                    <div class="file-upload">
                        <input
                            type="file"
                            id="template-file"
                            class="file-input"
                            accept=".docx,.doc,.pdf,.txt"
                            onchange={onfile}
                            required=true
                        />
                        <div class="file-upload-info">
                            {if let Some(selected_file) = (*file).clone() {
                                selected_file.name()
                            } else {
                                String::from("Файл не выбран")
                            }}
                        </div>
                    </div>
                </div>

                <button type="submit" class="btn btn-primary">
                    {"Загрузить шаблон"}
                </button>
            </form>
                    </div>
                </div>

                <div class="templates-list-column">
                    <h2>{"Список шаблонов"}</h2>
                    <div class="templates-list">
                        {(*templates).iter().map(|template| {
                            html! {
                                <div class="template-item" key={template.id}>
                                    <div class="template-name">{&template.name}</div>
                                    <div class="template-extension">{&template.extension}</div>
                                </div>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>
            </div>
        </div>
    }
} 