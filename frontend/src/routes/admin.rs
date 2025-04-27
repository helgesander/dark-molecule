use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum AdminRoute {
    #[at("/admin/users")]
    AdminUsers,
    #[at("/admin/user")]
    AdminUser,
    #[at("/admin/settings")]
    AdminSettings
}

pub fn switch_admin(route: AdminRoute) -> Html {
    match route {
        AdminRoute::AdminUser => html! { <h1>{"Admin User Show with id"}</h1> },
        AdminRoute::AdminUsers => html! { <h1>{"Admin Users Show"}</h1> },
        AdminRoute::AdminSettings => html! { <h1>{"Admin Settings"}</h1>}
    }
}