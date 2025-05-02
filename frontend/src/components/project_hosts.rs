use yew::prelude::*;
use crate::api::Host;

#[derive(Properties, PartialEq)]
pub struct ProjectHostsProps {
    pub hosts: Vec<Host>,
}

#[function_component(ProjectHosts)]
pub fn project_hosts(props: &ProjectHostsProps) -> Html {
    html! {
        <div class="hosts-section">
            <h2>{"Хосты"}</h2>
            <div class="hosts-grid">
                {for props.hosts.iter().map(|host| {
                    html! {
                        <div class="host-card">
                            <h3>{&host.name}</h3>
                            <div class="host-details">
                                <p class="ip"><strong>{"IP:"}</strong> {&host.ip}</p>
                                {if let Some(os) = &host.os {
                                    html! { <p class="os"><strong>{"OS:"}</strong> {os}</p> }
                                } else {
                                    html! {}
                                }}
                            </div>
                        </div>
                    }
                })}
            </div>
        </div>
    }
} 