use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScanStatus {
    Queued,
    Running,
    Completed,
    Failed,
}

impl ScanStatus {
    pub fn is_completed(&self) -> bool {
        matches!(self, ScanStatus::Completed)
    }
}