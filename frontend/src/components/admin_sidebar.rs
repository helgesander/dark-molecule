use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AdminSidebarProps {
    pub active_tab: String,
    pub on_tab_select: Callback<String>,
}

#[function_component(AdminSidebar)]
pub fn admin_sidebar(props: &AdminSidebarProps) -> Html {
    let tabs = vec![
        ("users", "Users", "/static/icons/users.svg"),
        ("settings", "Settings", "/static/icons/settings.svg"),
        ("teams", "Teams", "/static/icons/team.svg"),
    ];  
    

    html! {
        <aside class="sidebar">   
            <div class="sidebar-menu">
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
            </div>
        </aside>
    }
}