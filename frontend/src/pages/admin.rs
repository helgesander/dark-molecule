use yew::prelude::*;
use crate::components::{
    admin_sidebar::AdminSidebar,
    admin_users::AdminUsers,
    admin_teams::AdminTeams,
    admin_report_templates::AdminReportTemplates,
};

#[function_component(AdminPage)]
pub fn admin_page() -> Html {
    let active_tab = use_state(|| "users".to_string());

    let on_tab_select = {
        let active_tab = active_tab.clone();
        Callback::from(move |new_tab: String| {
            active_tab.set(new_tab);
        })
    };

    html! {
        <div class="project-layout">
            <AdminSidebar 
                active_tab={(*active_tab).clone()} 
                on_tab_select={on_tab_select}
            />
            <div class="project-main">
                {match active_tab.as_str() {
                    "users" => html! { <AdminUsers /> },
                    // "settings" => html! { <AdminSettings /> },
                    "teams" => html! { <AdminTeams /> },
                    "reports_templates" => html! { <AdminReportTemplates /> },
                    _ => html! { <AdminUsers /> },
                }}
            </div>
        </div>
    }
} 