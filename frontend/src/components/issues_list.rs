use yew::prelude::*;
use crate::api::Issue;
use uuid::Uuid;
use crate::components::severity_icon::SeverityIcon;
use crate::utils::calculate_severity;

#[derive(Properties, PartialEq)]
pub struct IssuesListProps {
    pub issues: Vec<Issue>,
    pub on_create_click: Callback<()>,
    pub on_issue_click: Callback<Uuid>,
}

#[function_component(IssuesList)]
pub fn issues_list(props: &IssuesListProps) -> Html {
    let error = use_state(|| String::new());
    
    html! {
        <>
            <div class="content">
                <div class="issues-list">
                    if !error.is_empty() {
                        <div class="error-message">{error.to_string()}</div>
                    }
                    
                    if props.issues.is_empty() {
                        <div class="empty-message">{"Нет уязвимостей"}</div>
                    } else {
                        <div class="issues-grid">
                            {for props.issues.iter().map(|issue| {
                                let id = issue.id;
                                let on_click = props.on_issue_click.clone();
                                let severity = calculate_severity(issue.cvss);

                                html! {
                                    <div 
                                        class="issue-card" 
                                        onclick={Callback::from(move |_| on_click.emit(id))}
                                    >
                                        <div class="issue-severity">
                                            <SeverityIcon severity={severity} />
                                        </div>
                                        <div class="issue-content">
                                            <h3>{&issue.name}</h3>
                                            {if let Some(desc) = &issue.description {
                                                html! { <p class="description">{desc}</p> }
                                            } else {
                                                html! {}
                                            }}
                                            <div class="issue-meta">
                                                <span class="cvss-badge">{format!("CVSS: {:.1}", issue.cvss)}</span>
                                            </div>
                                        </div>
                                    </div>
                                }
                            })}
                        </div>
                    }
                </div>
            </div>
        </>
    }
}
