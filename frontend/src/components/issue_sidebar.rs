use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IssueSidebarProps {
    pub active_tab: String,
    pub on_tab_select: Callback<String>,
}

#[function_component(IssueSidebar)]
pub fn issue_sidebar(props: &IssueSidebarProps) -> Html {
    let tabs = vec![
        ("back", "Назад", "/static/icons/back.svg"),
        ("edit", "Редактирование", "/static/icons/edit.svg"),
        ("poc", "PoC", "/static/icons/poc.svg"),
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