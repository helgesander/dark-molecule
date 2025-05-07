use yew::prelude::*;
use crate::api::{ApiClient, Issue, UpdateIssue};
use crate::debug_log;
use uuid::Uuid;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};

#[derive(Properties, PartialEq)]
pub struct EditIssueProps {
    pub project_id: Uuid,
    pub issue_id: Uuid,
}

#[function_component(EditIssuePage)]
pub fn edit_issue_page(props: &EditIssueProps) -> Html {
    let issue = use_state(|| None::<Issue>);
    let name = use_state(String::new);
    let description = use_state(String::new);
    let mitigation = use_state(String::new);
    let cvss = use_state(String::new);

    // Загружаем данные уязвимости
    {
        let issue = issue.clone();
        let name = name.clone();
        let description = description.clone();
        let mitigation = mitigation.clone();
        let cvss = cvss.clone();
        let project_id = props.project_id;
        let issue_id = props.issue_id;

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match ApiClient::get().get_issue(project_id, issue_id).await {
                    Ok(loaded_issue) => {
                        issue.set(Some(loaded_issue.clone()));
                        name.set(loaded_issue.name);
                        description.set(loaded_issue.description.unwrap_or_default());
                        mitigation.set(loaded_issue.mitigation.unwrap_or_default());
                        cvss.set(loaded_issue.cvss.to_string());
                    }
                    Err(e) => {
                        debug_log!("Failed to get issue: {}", e);
                    }
                }
            });
            || {}
        }, ());
    }

    // Обработчики изменений полей
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

    let on_mitigation_change = {
        let mitigation = mitigation.clone();
        Callback::from(move |e: Event| {
            let textarea = e.target_unchecked_into::<HtmlTextAreaElement>();
            mitigation.set(textarea.value());
        })
    };

    let on_cvss_change = {
        let cvss = cvss.clone();
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            cvss.set(input.value());
        })
    };

    // Обработчик отправки формы
    let on_submit = {
        let name = name.clone();
        let description = description.clone();
        let mitigation = mitigation.clone();
        let cvss = cvss.clone();
        let project_id = props.project_id;
        let issue_id = props.issue_id;

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            let updated_issue = UpdateIssue {
                name: (*name).clone(),
                description: Some((*description).clone()),
                mitigation: Some((*mitigation).clone()),
                cvss: (*cvss).parse().unwrap_or(0.0),
            };

            wasm_bindgen_futures::spawn_local(async move {
                match ApiClient::get()
                    .edit_issue(project_id, issue_id, updated_issue)
                    .await
                {
                    Ok(_) => {
                        debug_log!("Issue updated successfully");
                    },
                    Err(e) => {
                        debug_log!("Failed to update issue: {}", e);
                    }
                }
            });
        })
    };

    html! {
        <div class="edit-issue-form">
            <h1>{"Редактирование уязвимости"}</h1>
            
            <form onsubmit={on_submit}>
                <div class="form-group">
                    <label for="name">{"Название"}</label>
                    <input
                        type="text"
                        id="name"
                        class="form-control"
                        value={(*name).clone()}
                        onchange={on_name_change}
                        required=true
                    />
                </div>
                
                <div class="form-group">
                    <label for="description">{"Описание"}</label>
                    <textarea
                        id="description"
                        class="form-control"
                        value={(*description).clone()}
                        onchange={on_description_change}
                        rows="5"
                    />
                </div>
                
                <div class="form-group">
                    <label for="mitigation">{"Методы устранения"}</label>
                    <textarea
                        id="mitigation"
                        class="form-control"
                        value={(*mitigation).clone()}
                        onchange={on_mitigation_change}
                        rows="3"
                    />
                </div>
                
                <div class="form-group">
                    <label for="cvss">{"CVSS Score"}</label>
                    <input
                        type="number"
                        id="cvss"
                        class="form-control"
                        value={(*cvss).clone()}
                        onchange={on_cvss_change}
                        step="0.1"
                        min="0"
                        max="10"
                    />
                </div>
                
                <button type="submit" class="btn btn-primary">{"Сохранить изменения"}</button>
            </form>
        </div>
    }
}