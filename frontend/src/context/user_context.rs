use yew::prelude::*;
use std::rc::Rc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub username: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UserContext {
    pub user: Option<User>,
    pub set_user: Callback<Option<User>>,
}

impl Reducible for UserContext {
    type Action = Option<User>;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Self {
            user: action,
            set_user: self.set_user.clone(),
        }
        .into()
    }
}

pub fn use_user_context() -> impl Hook<Output = UseReducerHandle<UserContext>> {
    use_reducer(|| UserContext {
        user: None,
        set_user: Callback::default(),
    })
} 