use yew::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDate;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub scope: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub folder: String,
    pub team_id: Uuid,
}

#[derive(Properties, PartialEq)]
pub struct ProjectListProps {
    pub projects: Vec<Project>,
}

#[function_component(ProjectList)]
pub fn project_list(props: &ProjectListProps) -> Html {
    html! {
        <div class="project-list">
            <h2>{"Проекты"}</h2>
            <div class="projects-grid">
                {for props.projects.iter().map(|project| {
                    html! {
                        <div class="project-card" key={project.id.to_string()}>
                            <h3>{&project.name}</h3>
                            if let Some(description) = &project.description {
                                <p class="description">{description}</p>
                            }
                            <div class="project-details">
                                if let Some(scope) = &project.scope {
                                    <p class="scope"><strong>{"Область:"}</strong> {scope}</p>
                                }
                                if let Some(start_date) = &project.start_date {
                                    <p class="date"><strong>{"Начало:"}</strong> {start_date.format("%d.%m.%Y").to_string()}</p>
                                }
                                if let Some(end_date) = &project.end_date {
                                    <p class="date"><strong>{"Окончание:"}</strong> {end_date.format("%d.%m.%Y").to_string()}</p>
                                }
                            </div>
                        </div>
                    }
                })}
            </div>
        </div>
    }
} 