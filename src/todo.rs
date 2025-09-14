
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::form::form_inputs::enum_field::EnumDisplay;

#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq)]
pub enum TodoStatus {
    Completed,
    InProgress,
    Pending,
}

impl EnumDisplay for TodoStatus{
    fn to_str(&self) -> &str {
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
