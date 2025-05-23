use yew::prelude::*;
use crate::api::{ApiClient, Host};
use uuid::Uuid;
use crate::components::add_host_modal::AddHostModal;
use crate::components::scan_modal::ScanModal;
use crate::components::confirm_delete_modal::ConfirmDeleteModal;

#[derive(Properties, PartialEq)]
pub struct ProjectHostsProps {
    pub hosts: Vec<Host>,
    pub project_id: Uuid,
}

#[function_component(ProjectHosts)]
pub fn project_hosts(props: &ProjectHostsProps) -> Html {
    let hosts = use_state(|| props.hosts.clone());
    let selected_host = use_state(|| None::<Host>);
    let show_edit_host_modal = use_state(|| false);
    let show_delete_confirm_modal = use_state(|| false);
    let edit_host_header = use_state(|| "Добавление хоста".to_string());
    let show_add_modal = use_state(|| false);
    let show_scan_modal = use_state(|| false);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    let on_host_click = {
        let selected_host = selected_host.clone();
        let show_edit_host_modal = show_edit_host_modal.clone();
        let edit_host_header = edit_host_header.clone();
        Callback::from(move |host: Host| {
            selected_host.set(Some(host.clone()));
            show_edit_host_modal.set(true);
            edit_host_header.set("Редактирование хоста".to_string());
        })
    };

    let on_host_add_click = {
        let show_edit_host_modal = show_edit_host_modal.clone();
        let edit_host_header = edit_host_header.clone();
        let selected_host = selected_host.clone();
        Callback::from(move |_: MouseEvent| {
            selected_host.set(None);
            show_edit_host_modal.set(true);
            edit_host_header.set("Добавление хоста".to_string());
        })
    };

    let on_modal_close = {
        let show_edit_host_modal = show_edit_host_modal.clone();
        let selected_host = selected_host.clone();
        Callback::from(move |_: ()| {
            show_edit_host_modal.set(false);
            selected_host.set(None);
        })
    };

    let on_delete_click = {
        let selected_host = selected_host.clone();
        let show_delete_confirm_modal = show_delete_confirm_modal.clone();
        Callback::from(move |host: Host| {
            selected_host.set(Some(host));
            show_delete_confirm_modal.set(true);
        })
    };

    let on_delete_confirm = {
        let selected_host = selected_host.clone();
        let hosts = hosts.clone();
        let show_delete_confirm_modal = show_delete_confirm_modal.clone();
        let project_id = props.project_id;
        let error = error.clone();
        Callback::from(move |_| {
            if let Some(host) = (*selected_host).clone() {
                let hosts = hosts.clone();
                let show_delete_confirm_modal = show_delete_confirm_modal.clone();
                let project_id = project_id;
                let error = error.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    match ApiClient::get().delete_host(project_id, host.id).await {
                        Ok(_) => {
                            hosts.set(hosts.iter().filter(|h| h.id != host.id).cloned().collect());
                            show_delete_confirm_modal.set(false);
                        }
                        Err(e) => {
                            error.set(Some(e));
                        }
                    }
                });
            }
        })
    };

    let on_delete_cancel = {
        let show_delete_confirm_modal = show_delete_confirm_modal.clone();
        let selected_host = selected_host.clone();
        Callback::from(move |_| {
            show_delete_confirm_modal.set(false);
            selected_host.set(None);
        })
    };

    html! {
        <div class="hosts-section">
            <div class="hosts-header">
                <h2>{"Хосты"}</h2>
                <div class="hosts-actions">
                    <button class="btn btn-primary" onclick={on_host_add_click.clone()}>
                        <img src="/static/icons/plus.svg" class="icon" alt="Добавить" />
                        {"Добавить"}
                    </button>
                    <button class="btn btn-primary">
                        <img src="/static/icons/plus.svg" class="icon" alt="Добавить с помощью сканера" />
                        {"Добавить с помощью сканера"}
                    </button>
                </div>
            </div>
            <div class="hosts-grid">
                {for hosts.iter().map(|host| {
                    let on_click = on_host_click.clone();
                    let on_delete = on_delete_click.clone();
                    let host_for_click = host.clone();
                    let host_for_delete = host.clone();
                    let host_for_display = host.clone();
                    html! {
                        <div class="host-card" onclick={Callback::from(move |_| on_click.emit(host_for_click.clone()))}>
                            <div class="host-content">
                                <h3>{host_for_display.hostname.as_deref().unwrap_or("Без имени")}</h3>
                                <div class="host-details">
                                    <p class="ip"><strong>{"IP:"}</strong> {&host_for_display.ip_address}</p>
                                </div>
                            </div>
                            <div class="host-actions">
                                <button class="btn btn-icon" onclick={Callback::from(move |_| {
                                    on_delete.emit(host_for_delete.clone());
                                })}>
                                    <img src="/static/icons/trash.svg" class="icon" alt="Удалить" />
                                </button>
                            </div>
                        </div>
                    }
                })}
            </div>
            if *show_edit_host_modal {
                <AddHostModal
                    project_id={props.project_id}
                    on_close={on_modal_close.clone()}
                    header={(*edit_host_header).clone()}
                    host={(*selected_host).clone()}
                />
            }

            if *show_scan_modal {
                <ScanModal
                    project_id={props.project_id}
                    on_close={on_modal_close.clone()}
                    scan_type={"nmap".to_string()}
                />
            }

            if *show_delete_confirm_modal {
                <ConfirmDeleteModal
                    title={"Удаление хоста".to_string()}
                    message={"Вы уверены, что хотите удалить этот хост? Это действие нельзя отменить.".to_string()}
                    on_confirm={on_delete_confirm}
                    on_cancel={on_delete_cancel}
                />
            }
        </div>
    }
}