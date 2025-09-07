
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub enum TodoStatus {
    Completed,
    InProgress,
    Pending,
}

impl TodoStatus{
    pub fn to_str(&self) -> &str {
        match self {
            TodoStatus::Completed => "Completed",
            TodoStatus::InProgress => "InProgress",
            TodoStatus::Pending => "Pending",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TodoItem {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub status: TodoStatus,
}
