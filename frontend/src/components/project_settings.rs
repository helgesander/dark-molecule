use yew::prelude::*;
use uuid::Uuid;
#[derive(Properties, PartialEq)]
pub struct ProjectSettingsProps {
    pub project_id: Uuid,
}

#[function_component(ProjectSettings)]
pub fn project_settings(props: &ProjectSettingsProps) -> Html {
    html! {
        <div>
            <h1>{"Настройки проекта"}</h1>
        </div>
    }
}
