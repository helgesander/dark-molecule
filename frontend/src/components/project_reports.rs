use yew::prelude::*;
use crate::api::{Report, ApiClient};
use uuid::Uuid;

#[derive(Properties, PartialEq)]
pub struct ProjectReportsProps {
    pub reports: Vec<Report>,
    pub project_id: Uuid,
}

#[function_component(ProjectReports)]
pub fn project_reports(props: &ProjectReportsProps) -> Html {
    let reports = use_state(|| props.reports.clone());

    html! {
        <div class="reports-section">
            <h2>{"Отчеты"}</h2>
            <div class="reports-grid">
                {
                    if reports.is_empty() {
                        html! { <div class="no-reports">{"Отчетов нет"}</div> }
                    } else {
                        html! {
                            <>
                                {for reports.iter().map(|report| {
                                    html! {
                                        <div class="report-card">
                                            <h3>{&report.name}</h3>
                                        </div>
                                    }
                                })}
                            </>
                        }
                    }
                }
            </div>
        </div>
    }
}