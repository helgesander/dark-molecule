use yew::prelude::*;
use crate::context::user_context::{UserContext, create_user_context};

#[derive(Properties, PartialEq)]
pub struct UserProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(UserProvider)]
pub fn user_provider(props: &UserProviderProps) -> Html {
    let user_ctx = use_reducer(|| create_user_context());

    html! {
        <ContextProvider<UserContext> context={user_ctx}>
            {props.children.clone()}
        </ContextProvider<UserContext>>
    }
} 