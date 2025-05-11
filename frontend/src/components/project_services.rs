use yew::prelude::*;
use crate::api::Service;
use uuid::Uuid;
use crate::debug_log;

#[derive(Properties, PartialEq)]
pub struct ProjectServicesProps {
    pub services: Vec<Service>,
}

#[function_component(ProjectServices)]
pub fn project_services(props: &ProjectServicesProps) -> Html {
    let services = use_state(|| props.services.clone());
    let on_service_click = {
        Callback::from(move |service_id: Uuid| {
            debug_log!("Service clicked: {}", service_id);
        })
    };

    html! {
        <div>
            <h1>{"Services"}</h1>
            <h1 align="center">{"IN DEVELOPMENT"}</h1>
            // <ul>
            //     {for services.iter().map(|service| {
            //         let service_id = service.id;
            //         let on_click = on_service_click.clone();
            //         html! { 
            //             <li onclick={Callback::from(move |_: MouseEvent| on_click.emit(service_id))}>
            //                 {&service.name}
            //             </li> 
            //         }
            //     })}
            // </ul>
        </div>
    }
}