use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Local};

#[derive(Serialize, Deserialize)]
pub struct Report {
    id: Uuid,
    name: String,
    date: String, // DateTime<Local>,
    text: String
}

impl Report {
    pub fn new(name: String, text: String) -> Report {
        Report {
            id: Uuid::new_v4(),
            name,
            date: String::new(), // Local::now(),
            text
        }
    }
    pub fn get_id(&self) -> Uuid {
        self.id
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_date(&self) -> String {
        self.date.clone()
    }
    pub fn get_text(&self) -> String {
        self.text.clone()
    }
}