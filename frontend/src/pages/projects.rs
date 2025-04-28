use yew::prelude::*;
use gloo::console::log;
use gloo_net::http::Request;
use web_sys::RequestCredentials;
use crate::components::navbar::Navbar;
use crate::components::footer::Footer;

#[derive(serde::Deserialize, Clone)]
struct Project {
    id: String,
    name: String,
    scope: String,
}

#[function_component(ProjectsPage)]
pub fn projects_page() -> Html {
    let projects = use_state(|| Vec::<Project>::new());
    let error = use_state(|| String::new());

    {
        let projects = projects.clone();
        let error = error.clone();
        use_effect_with((), move |_| {
            let projects = projects.clone();
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get("http://localhost:8000/api/project/")
                    .credentials(RequestCredentials::Include)
                    .send()
                    .await;

                match response {
                    Ok(resp) => {
                        if resp.ok() {
                            let data = resp.json::<Vec<Project>>().await.unwrap();
                            projects.set(data);
                        } else {
                            error.set("Ошибка при загрузке проектов".to_string());
                        }
                    }
                    Err(e) => {
                        log!("Error loading projects:", format!("{:?}", e));
                        error.set("Ошибка при загрузке проектов".to_string());
                    }
                }
            });
        });
    }

    html! {
        <>
            <Navbar />
            <div class="projects-container">
                <h1>{"Проекты"}</h1>
                if !error.is_empty() {
                    <div class="error-message">{error.to_string()}</div>
                }
                <div class="projects-list">
                    {for projects.iter().map(|project| {
                        html! {
                            <div class="project-card">
                                <h2>{&project.name}</h2>
                                <p>{&project.scope}</p>
                            </div>
                        }
                    })}
                </div>
            </div>
            <Footer />
        </>
    }
} 