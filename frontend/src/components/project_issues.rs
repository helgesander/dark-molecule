use yew::prelude::*;
use crate::api::{Issue, ApiClient};
use crate::routes::project::ProjectRoute;
use uuid::Uuid;
use yew_router::prelude::use_navigator;
use crate::components::issues_list::IssuesList;
use crate::components::issue_create_form::IssueCreateForm;
use wasm_bindgen_futures;
use crate::components::scan_modal::ScanModal;
use web_sys::MouseEvent;
use crate::debug_log;

#[derive(Properties, PartialEq)]
pub struct ProjectIssuesProps {
    #[prop_or_default]
    pub issues: Vec<Issue>,
    pub project_id: Uuid,
}

#[function_component(ProjectIssues)]
pub fn project_issues(props: &ProjectIssuesProps) -> Html {
    let issues = use_state(|| props.issues.clone());
    let navigator = use_navigator().unwrap();
    let show_create_form = use_state(|| false);
    let show_scan_modal = use_state(|| false);

    let on_create_issue = {
        let issues = issues.clone();
        let project_id = props.project_id;
        let show_create_form = show_create_form.clone();
        Callback::from(move |_| {
            show_create_form.set(true);
        })
    };

    let on_scan_hosts_click = {
        let show_scan_modal = show_scan_modal.clone();
        Callback::from(move |_: MouseEvent| {
            show_scan_modal.set(true);
        })
    };

    let on_scan_modal_close = {
        let show_scan_modal = show_scan_modal.clone();
        Callback::from(move |_: ()| {
            show_scan_modal.set(false);
        })
    };

    let on_cancel = {
        let show_create_form = show_create_form.clone();
        Callback::from(move |_| {
            show_create_form.set(false);
        })
    };

    let on_submit = {
        let issues = issues.clone();
        let project_id = props.project_id;
        let show_create_form = show_create_form.clone();
        Callback::from(move |name: String| {
            let issues = issues.clone();
            let show_create_form = show_create_form.clone();
            wasm_bindgen_futures::spawn_local(async move {
                show_create_form.set(false);
                match ApiClient::get().create_issue(project_id, name).await {
                    Ok(issue) => {
                        issues.set({
                            let mut issues = (*issues).clone();
                            issues.push(issue);
                            issues
                        });
                    }
                    Err(e) => {
                        debug_log!("Error creating issue: {}", e);
                    }
                }
            });
        })
    };

    let on_issue_click = {
        let issues = issues.clone();
        let project_id = props.project_id;
        Callback::from(move |issue_id: Uuid| {
            let navigator = navigator.clone();
            navigator.push(&ProjectRoute::EditIssue { id: project_id, issue_id });
        })
    };


    html! {
        <div class="issues-section">
            <div class="issues-header">
                {if *show_create_form {
                    html! {
                        <IssueCreateForm on_submit={on_submit} on_cancel={on_cancel} />
                    }
                } else {
                    html! {
                        <>
                        <button class="create-issue-button" onclick={Callback::from(move |_| show_create_form.set(true))}>
                            <img src="/static/icons/plus.svg" class="icon" alt="Создать" />
                            {"Добавить"}
                        </button>
                        <button class="btn btn-primary" onclick={on_scan_hosts_click.clone()}>
                            <img src="/static/icons/plus.svg" class="icon" alt="Создать" />
                            {"Создать с помощью сканера"}
                        </button>
                        </>
                    }
                }}
            </div>
            <IssuesList issues={(*issues).clone()} on_create_click={on_create_issue} on_issue_click={on_issue_click} />
            if *show_scan_modal {
                <ScanModal
                    project_id={props.project_id}
                    on_close={on_scan_modal_close.clone()}
                    scan_type={"nuclei".to_string()}
                />
            }
        </div>
    }
}