use yew::prelude::*;
use crate::api::{Issue, ApiClient};
use crate::routes::project::ProjectRoute;
use uuid::Uuid;
use yew_router::prelude::use_navigator;
use crate::components::issues_list::IssuesList;
use crate::components::issue_create_form::IssueCreateForm;
use wasm_bindgen_futures;
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

    let on_create_issue = {
        let issues = issues.clone();
        let project_id = props.project_id;
        let show_create_form = show_create_form.clone();
        Callback::from(move |_| {
            show_create_form.set(true);
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

    // let on_name_change = {
    //     let new_issue_name = new_issue_name.clone();
    //     Callback::from(move |e: InputEvent| {
    //         if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
    //             new_issue_name.set(input.value());
    //         }
    //     })
    // };

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
                        <button class="btn btn-primary">
                            <img src="/static/icons/plus.svg" class="icon" alt="Создать" />
                            {"Создать с помощью"}
                        </button>
                        </>
                    }
                }}
            </div>
            <IssuesList issues={(*issues).clone()} on_create_click={on_create_issue} on_issue_click={on_issue_click} />
        </div>
    }
}