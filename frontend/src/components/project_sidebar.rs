use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProjectSidebarProps {
    pub active_tab: String,
    pub on_tab_select: Callback<String>,
    pub project_name: String,
}

#[function_component(ProjectSidebar)]
pub fn project_sidebar(props: &ProjectSidebarProps) -> Html {
    let tabs = vec![
        ("hosts", "Хосты", "/static/icons/hosts.svg"),
        ("issues", "Issues", "/static/icons/issues.svg"),
        ("reports", "Reports", "/static/icons/reports.svg"),
        ("services", "Services", "/static/icons/services.svg"),
        ("settings", "Settings", "/static/icons/settings.svg"),
    ];

    html! {
        <aside class="sidebar">
            // <div class="sidebar-header">
            //     {props.project_name.clone()}
            // </div>
            <nav class="sidebar-menu">
                <ul>
                    {for tabs.iter().map(|(key, label, icon)| {
                        let is_active = props.active_tab == *key;
                        let on_tab_select = props.on_tab_select.clone();
                        let key = key.to_string();
                        html! {
                            <li class={if is_active { "active" } else { "" }} onclick={Callback::from(move |_| on_tab_select.emit(key.clone()))}>
                                <img src={icon.to_string()} class="icon" alt={label.to_string()} />
                            </li>
                        }
                    })}
                </ul>
            </nav>
        </aside>
    }
} 