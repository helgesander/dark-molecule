use yew::prelude::*;
use crate::api::{ApiClient, IssueFullResponse, UpdateIssue, Host};
use crate::debug_log;
use uuid::Uuid;
use web_sys::{HtmlInputElement, HtmlTextAreaElement, HtmlSelectElement};
use gloo::timers::callback::Timeout;
use crate::components::issue_sidebar::IssueSidebar;
use crate::routes::project::ProjectRoute;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EditIssueProps {
    pub project_id: Uuid,
    pub issue_id: Uuid,
}

#[function_component(EditIssuePage)]
pub fn edit_issue_page(props: &EditIssueProps) -> Html {
    let issue = use_state(|| None::<IssueFullResponse>);
    let name = use_state(String::new);
    let description = use_state(String::new);
    let mitigation = use_state(String::new);
    let cvss = use_state(String::new);
    let show_success = use_state(|| false);
    let active_tab = use_state(|| "edit".to_string());
    let navigator = use_navigator().unwrap();
    let hosts: UseStateHandle<Vec<Host>> = use_state(Vec::new);
    let selected_hosts: UseStateHandle<Vec<Host>> = use_state(Vec::new);

    let on_host_select = {
        let selected_hosts = selected_hosts.clone();
        Callback::from(move |host: Host| {
            let mut current = (*selected_hosts).clone();
            if current.contains(&host) {
                current.retain(|h| h != &host);
            } else {
                current.push(host);
            }
            selected_hosts.set(current);
        })
    };

    // Загружаем список хостов
    {
        let hosts = hosts.clone();
        let project_id = props.project_id;

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match ApiClient::get().get_hosts(project_id).await {
                    Ok(loaded_hosts) => {
                        hosts.set(loaded_hosts);
                    }
                    Err(e) => {
                        debug_log!("Failed to get hosts: {}", e);
                    }
                }
            });
            || {}
        }, ());
    }

    let on_tab_select = {
        let active_tab = active_tab.clone();
        let navigator = navigator.clone();
        let project_id = props.project_id;
        Callback::from(move |tab: String| {
            if tab == "back" {
                navigator.push(&ProjectRoute::Project { id: project_id });
            } else {
                active_tab.set(tab);
            }
        })
    };

    // Загружаем данные уязвимости
    {
        let issue = issue.clone();
        let name = name.clone();
        let description = description.clone();
        let mitigation = mitigation.clone();
        let cvss = cvss.clone();
        let selected_hosts = selected_hosts.clone();
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
                        selected_hosts.set(loaded_issue.hosts);
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
        let selected_hosts = selected_hosts.clone();
        let project_id = props.project_id;
        let issue_id = props.issue_id;
        let show_success = show_success.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            let updated_issue = UpdateIssue {
                name: (*name).clone(),
                description: Some((*description).clone()),
                mitigation: Some((*mitigation).clone()),
                cvss: (*cvss).parse().unwrap_or(0.0),
                hosts: (*selected_hosts).clone(),
            };

            let show_success = show_success.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match ApiClient::get()
                    .edit_issue(project_id, issue_id, updated_issue)
                    .await
                {
                    Ok(_) => {
                        debug_log!("Issue updated successfully");
                        show_success.set(true);
                        let show_success = show_success.clone();
                        let timeout = Timeout::new(5_000, move || {
                            show_success.set(false);
                        });
                        timeout.forget();
                    },
                    Err(e) => {
                        debug_log!("Failed to update issue: {}", e);
                    }
                }
            });
        })
    };

    html! {
        <div class="project-layout">
            <IssueSidebar active_tab={(*active_tab).clone()} on_tab_select={on_tab_select} />
            <main class="project-main">
                {match active_tab.as_str() {
                    "edit" => html! {
                        <div class="edit-issue-form">
                            <h1>{"Редактирование уязвимости"}</h1>
                            if *show_success {
                                <div class="success-message">{"Изменения сохранены"}</div>
                            }
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

                                <div class="form-group">
                                    <label>{"Затронутые хосты"}</label>
                                    <div class="hosts-select">
                                        <select 
                                            multiple=true
                                            onchange={
                                                let selected_hosts = selected_hosts.clone();
                                                let hosts = hosts.clone();
                                                Callback::from(move |e: Event| {
                                                    let select = e.target_unchecked_into::<HtmlSelectElement>();
                                                    let options = select.selected_options();
                                                    let mut new_selected: Vec<Host> = Vec::new();
                                                    
                                                    for i in 0..options.length() {
                                                        if let Some(option) = options.item(i) {
                                                            if let Some(index) = option.get_attribute("value")
                                                                .and_then(|v| v.parse::<usize>().ok()) {
                                                                if let Some(host) = hosts.get(index) {
                                                                    new_selected.push((*host).clone());
                                                                }
                                                            }
                                                        }
                                                    }
                                                    selected_hosts.set(new_selected);
                                                })
                                            }
                                            class="form-control"
                                        >
                                            {for hosts.iter().enumerate().map(|(index, host)| {
                                                let is_selected = (*selected_hosts).contains(host);
                                                let display_name = match &host.hostname {
                                                    Some(hostname) => format!("{} ({})", hostname, host.ip_address),
                                                    None => host.ip_address.clone(),
                                                };
                                                html! {
                                                    <option 
                                                        value={index.to_string()}
                                                        selected={is_selected}
                                                    >
                                                        {display_name}
                                                    </option>
                                                }
                                            })}
                                        </select>
                                        <div class="selected-hosts">
                                            <h4>{"Выбранные хосты:"}</h4>
                                            {if (*selected_hosts).is_empty() {
                                                html! { <p class="no-hosts">{"Хосты не выбраны"}</p> }
                                            } else {
                                                html! {
                                                    <ul>
                                                        {for selected_hosts.iter().map(|host| {
                                                            let display_name = match &host.hostname {
                                                                Some(hostname) => format!("{} ({})", hostname, host.ip_address),
                                                                None => host.ip_address.clone(),
                                                            };
                                                            html! { <li>{display_name}</li> }
                                                        })}
                                                    </ul>
                                                }
                                            }}
                                        </div>
                                    </div>
                                </div>
                                
                                <button type="submit" class="btn btn-primary">{"Сохранить изменения"}</button>
                            </form>
                        </div>
                    },
                    "poc" => html! {
                        <div class="poc-section">
                            <h1>{"PoC"}</h1>
                            <p>{"Здесь будет содержимое для PoC"}</p>
                        </div>
                    },
                    _ => html! {
                        <div>{"Неизвестная вкладка"}</div>
                    }
                }}
            </main>
        </div>
    }
}