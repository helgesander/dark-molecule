use yew::prelude::*;
use crate::api::{Report, ApiClient};
use uuid::Uuid;
use crate::components::report_form::ReportForm;

#[derive(Properties, PartialEq)]
pub struct ProjectReportsProps {
    pub reports: Vec<Report>,
    pub project_id: Uuid,
}

#[function_component(ProjectReports)]
pub fn project_reports(props: &ProjectReportsProps) -> Html {
    let reports = use_state(|| props.reports.clone());

    html! {
        <div class="reports-container">
            <div class="reports-left">
                <ReportForm project_id={props.project_id}/>
            </div>
            <div class="reports-right">
                <h3>{"Существующие отчеты"}</h3>
                if reports.is_empty() {
                    <p>{"Нет доступных отчетов"}</p>
                } else {
                    <ul class="reports-list">
                        {reports.iter().map(|report| {
                            html! {
                                <li class="report-item">
                                    <span class="report-name">{&report.name}</span>
                                </li>
                            }
                        }).collect::<Html>()}
                    </ul>
                }
            </div>
        </div>
    }
}