use yew::prelude::*;
use web_sys::{HtmlInputElement, HtmlTextAreaElement, HtmlSelectElement};
use wasm_bindgen::JsCast;
use crate::api::{CreateReportRequest, ApiClient, ReportTemplatePreview};
use uuid::Uuid;
use crate::debug_log;

#[derive(Properties, PartialEq)]
pub struct ReportFormProps {
    pub project_id: Uuid,
}

#[derive(Clone, PartialEq)]
struct Template {
    id: String,
    name: String,
    description: Option<String>,
}

#[function_component(ReportForm)]
pub fn report_form(props: &ReportFormProps) -> Html {
    let name = use_state(String::new);
    let description = use_state(String::new);
    let selected_template = use_state(|| None::<i32>);
    let show_success = use_state(|| false);
    let error_message = use_state(|| None::<String>);
    let templates = use_state(|| Vec::<ReportTemplatePreview>::new());

    // Загружаем шаблоны при монтировании компонента
    {
        let templates = templates.clone();
        let error_message = error_message.clone();
        
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match ApiClient::get().get_report_templates().await {
                    Ok(loaded_templates) => {
                        templates.set(loaded_templates);
                    }
                    Err(e) => {
                        error_message.set(Some(format!("Ошибка при загрузке шаблонов: {}", e)));
                    }
                }
            });
            || {}
        }, ());
    }

    let on_name_change = {
        let name = name.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            name.set(input.value());
        })
    };

    let on_description_change = {
        let description = description.clone();
        Callback::from(move |e: Event| {
            let textarea = e.target_unchecked_into::<HtmlTextAreaElement>();
            description.set(textarea.value());
        })
    };

    let on_template_change = {
        let selected_template = selected_template.clone();
        Callback::from(move |e: Event| {
            let select = e.target_unchecked_into::<HtmlSelectElement>();
            let value = select.value();
            if !value.is_empty() {
                if let Ok(id) = value.parse::<i32>() {
                    selected_template.set(Some(id));
                }
            } else {
                selected_template.set(None);
            }
        })
    };

    let on_submit = {
        let selected_template = selected_template.clone();
        let show_success = show_success.clone();
        let error_message = error_message.clone();
        let project_id = props.project_id;

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            if let Some(template_id) = *selected_template {
                let show_success = show_success.clone();
                let error_message = error_message.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    match ApiClient::get()
                        .create_report(project_id, template_id)
                        .await
                    {
                        Ok(_) => {
                            show_success.set(true);
                            error_message.set(None);
                            // Сбросить форму через 3 секунды
                            let show_success = show_success.clone();
                            let timeout = gloo::timers::callback::Timeout::new(3_000, move || {
                                show_success.set(false);
                            });
                            timeout.forget();
                        }
                        Err(e) => {
                            error_message.set(Some(format!("Ошибка при создании отчета: {}", e)));
                        }
                    }
                });
            } else {
                error_message.set(Some("Выберите шаблон отчета".to_string()));
            }
        })
    };

    html! {
        <div class="report-form">
            <h2>{"Создание отчета"}</h2>
            
            if *show_success {
                <div class="success-message">
                    {"Отчет успешно создан"}
                </div>
            }

            if let Some(error) = (*error_message).as_ref() {
                <div class="error-message">
                    {error}
                </div>
            }

            <form onsubmit={on_submit}>
            //     <div class="form-group">
            //         <label for="name">{"Название отчета"}</label>
            //         <input
            //             type="text"
            //             id="name"
            //             class="form-control"
            //             value={(*name).clone()}
            //             onchange={on_name_change}
            //             required=true
            //             placeholder="Введите название отчета"
            //         />
            //     </div>

                // <div class="form-group">
                //     <label for="description">{"Описание"}</label>
                //     <textarea
                //         id="description"
                //         class="form-control"
                //         value={(*description).clone()}
                //         onchange={on_description_change}
                //         rows="3"
                //         placeholder="Добавьте описание отчета"
                //     />
                // </div>

                <div class="form-group">
                    <label for="template">{"Шаблон отчета"}</label>
                    <select
                        id="template"
                        class="form-control"
                        onchange={on_template_change}
                        value={(*selected_template).map_or(String::new(), |v| v.to_string())}
                        required=true
                    >
                        <option value="">{"Выберите шаблон"}</option>
                        {for templates.iter().map(|template| {
                            html! {
                                <option value={template.id.to_string()}>
                                    {&template.name}
                                </option>
                            }
                        })}
                    </select>
                    // {if let Some(template) = templates.iter().find(|t| Some(t.id.parse::<i32>().unwrap_or(0)) == *selected_template) {
                    //     html! {
                    //         <div class="template-description">
                    //             {template.name.clone()}
                    //         </div>
                    //     }
                    // } else {
                    //     html! {}
                    // }}
                </div>

                <button type="submit" class="btn btn-primary">
                    {"Создать отчет"}
                </button>
            </form>
        </div>
    }
} 