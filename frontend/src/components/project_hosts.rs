use yew::prelude::*;
use crate::api::Host;
use uuid::Uuid;
use crate::debug_log;
use crate::components::add_host_modal::AddHostModal;

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
    let edit_host_header = use_state(|| "Добавление хоста".to_string());

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

    html! {
        <div class="hosts-section">
            <div class="hosts-header">
                <h2>{"Хосты"}</h2>
                <div class="hosts-actions">
                    <button class="btn btn-primary" onclick={on_host_add_click.clone()}>
                        <img src="/static/icons/plus.svg" class="icon" alt="Добавить" />
                        {"Добавить"}
                    </button>
                    // <button class="btn btn-primary">
                    //     <img src="/static/icons/scanner.svg" class="icon" alt="Добавить с помощью сканера" />
                    //     {"Добавить с помощью сканера"}
                    // </button>
                </div>
            </div>
            <div class="hosts-grid">
                {for hosts.iter().map(|host| {
                    let on_click = on_host_click.clone();
                    let host_for_click = host.clone();
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
                                <button class="btn btn-icon">
                                    <img src="/static/icons/edit.svg" class="icon" alt="Редактировать" />
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
        </div>
    }
} 