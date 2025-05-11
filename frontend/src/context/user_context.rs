use yew::prelude::*;
use std::rc::Rc;
use serde::{Deserialize, Serialize};
use gloo::console::log;
use crate::api;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Default, Deserialize, Serialize)]
pub struct User {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub is_admin: Option<bool>,
    pub avatar: Option<String>,
}

pub type UserContext = UseReducerHandle<User>;

impl User {
    // TODO: peredelat' nahuy
    pub fn is_all_none(&self) -> bool {
        let res = self.id.is_none() && self.username.is_none() && self.email.is_none() && self.is_admin.is_none();
        // log!("UserContext: is_all_none:", format!("{:?}", res));
        res
    }
}

impl Reducible for User {
    type Action = User;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let new_state = Self {
            id: action.id,
            username: action.username,
            email: action.email,
            is_admin: action.is_admin,
            avatar: action.avatar,
        };
        log!("UserContext: new state:", format!("{:?}", new_state));
        new_state.into()
    }
}

pub fn create_user_context() -> User {
    log!("UserContext: creating new context");
    User {
        id: None,
        username: None,
        email: None,
        is_admin: None,
        avatar: None,
    }
}

pub fn from_api_to_context(user: api::User) -> User {
    User {
        id: user.id,
        username: Some(user.username),
        email: Some(user.email),
        is_admin: Some(user.is_admin),
        avatar: user.avatar
    }
}
