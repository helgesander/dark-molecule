use yew::prelude::*;
use crate::api::{ApiClient, ReportPreview};
use uuid::Uuid;
use crate::components::report_form::ReportForm;
use wasm_bindgen_futures;
use web_sys::{Blob, Url, HtmlAnchorElement};
use wasm_bindgen::JsCast;
use js_sys::{Uint8Array, Array};

#[derive(Properties, PartialEq)]
pub struct ProjectReportsProps {
    pub reports: Vec<ReportPreview>,
    pub project_id: Uuid,
}

#[function_component(ProjectReports)]
pub fn project_reports(props: &ProjectReportsProps) -> Html {
    let reports = use_state(|| props.reports.clone());
    let project_id = props.project_id;

    {
        let reports = reports.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(loaded_reports) = ApiClient::get().get_reports_preview(project_id).await {
                    reports.set(loaded_reports);
                }
            });
            || {}
        }, ());
    }

    let on_report_click = {
        let project_id = project_id;
        Callback::from(move |report_id: i32| {
            let project_id = project_id;
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(report_data) = ApiClient::get().download_report(project_id, report_id).await {
                    let array = Uint8Array::new_with_length(report_data.data.len() as u32);
                    array.copy_from(&report_data.data);
                    let blob = Blob::new_with_u8_array_sequence(&Array::of1(&array))
                        .expect("Failed to create blob");

                    let url = Url::create_object_url_with_blob(&blob)
                        .expect("Failed to create object URL");

                    let window = web_sys::window().expect("No window found");
                    let document = window.document().expect("No document found");
                    let anchor = document.create_element("a")
                        .expect("Failed to create anchor element")
                        .dyn_into::<HtmlAnchorElement>()
                        .expect("Failed to cast to HtmlAnchorElement");

                    anchor.set_href(&url);
                    anchor.set_download(&report_data.filename);
                    anchor.style().set_property("display", "none").expect("Failed to set style");

                    document.body().expect("No body found").append_child(&anchor).expect("Failed to append anchor");
                    anchor.click();

                    document.body().expect("No body found").remove_child(&anchor).expect("Failed to remove anchor");
                    Url::revoke_object_url(&url).expect("Failed to revoke object URL");
                }
            });
        })
    };

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
                            let report_id = report.id;
                            let on_click = {
                                let on_report_click = on_report_click.clone();
                                Callback::from(move |_| on_report_click.emit(report_id))
                            };
                            html! {
                                <li class="report-item" onclick={on_click}>
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