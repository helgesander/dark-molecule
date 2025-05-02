use yew::prelude::*;
use crate::api::Issue;

#[derive(Properties, PartialEq)]
pub struct ProjectIssuesProps {
    #[prop_or_default]
    pub issues: Vec<Issue>,
}

#[function_component(ProjectIssues)]
pub fn project_issues(props: &ProjectIssuesProps) -> Html {
    html! {
        <div class="issues-section">
            <h2>{"Уязвимости"}</h2>
            <div class="issues-grid">
                {for props.issues.iter().map(|issue| {
                    html! {
                        <div class="issue-card">
                            <h3>{&issue.title}</h3>
                            <p class="description">{&issue.description}</p>
                            <div class="issue-meta">
                                <span class={format!("severity {}", issue.severity.to_lowercase())}>
                                    {&issue.severity}
                                </span>
                                <span class={format!("status {}", issue.status.to_lowercase())}>
                                    {&issue.status}
                                </span>
                            </div>
                        </div>
                    }
                })}
            </div>
        </div>
    }
} 