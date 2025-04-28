use yew::prelude::*;
use crate::context::user_context::{User, UserContext};

#[derive(Properties, PartialEq)]
pub struct UserProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(UserProvider)]
pub fn user_provider(props: &UserProviderProps) -> Html {
    let user = use_state(|| None::<User>);
    let set_user = {
        let user = user.clone();
        Callback::from(move |new_user: Option<User>| {
            user.set(new_user);
        })
    };

    let context = UserContext {
        user: (*user).clone(),
        set_user,
    };

    html! {
        <ContextProvider<UserContext> context={context}>
            {props.children.clone()}
        </ContextProvider<UserContext>>
    }
} 