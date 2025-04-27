use yew::prelude::*;
use crate::context::user_context::{User, UserContext};

#[function_component(UserProvider)]
pub fn user_provider(props: &Props) -> Html {
    let user = use_state(|| None);
    let set_user = {
        let user = user.clone();
        Callback::from(move |new_user: Option<User>| {
            user.set(new_user);
        })
    };

    let context = UserContext::new((*user).clone(), set_user);

    html! {
        <ContextProvider<UserContext> context={context}>
            {props.children.clone()}
        </ContextProvider<UserContext>>
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
} 