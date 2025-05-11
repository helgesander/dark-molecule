use yew::prelude::*;
use yew_router::prelude::*;


// TODO: to remove after
#[derive(Clone, Routable, PartialEq)]
pub enum AdminRoute {
    #[at("/admin/users")] // TODO: to remove after
    Users,
    #[at("/admin/user")] 
    CreateUser,
    #[at("/admin/settings")] // TODO: to remove after
    Settings
}

pub fn switch_admin(route: AdminRoute) -> Html {
    match route {
        AdminRoute::Users => html! { <h1>{"Admin User Show with id"}</h1> },
        AdminRoute::CreateUser => html! { <h1>{"Admin Users Show"}</h1> },
        AdminRoute::Settings => html! { <h1>{"Admin Settings"}</h1>}
    }
}