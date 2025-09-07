use std::fs;
use ratatui::widgets::ListState;

use crate::todo::{TodoItem};

#[derive(Debug, Default)]
pub struct TodoList {
    pub items: Vec<TodoItem>,
    pub state: ListState,
}

impl TodoList {
    pub fn read_todos(&mut self) {
        let path = ".\\data\\data.json";
        let data = fs::read_to_string(path).expect("Unable to read file");
        if data.is_empty() {
            self.items = Vec::new();
            self.state = ListState::default();
            return;
        }
        let todos: Vec<TodoItem> = serde_json::from_str(&data).expect("JSON was not well-formatted");
        
        self.items = todos;
        self.state = ListState::default();
    }

    pub fn save_todos(&self) {
        let path = ".\\data\\data.json";
        let data = serde_json::to_string(&self.items).expect("Unable to serialize data");
        fs::write(path, data).expect("Unable to write file");
    }

    pub fn remove_selected(&mut self) {
        if let Some(selected) = self.state.selected() {
            self.items.remove(selected);
            let new_len = self.items.len();
            if new_len == 0 {
                self.state.select(None);
            } else if selected >= new_len {
                self.state.select(Some(new_len - 1));
            } else {
                self.state.select(Some(selected));
            }
        }
    }

    pub fn mark_completed(&mut self) {
        if let Some(selected) = self.state.selected() {
            if let Some(item) = self.items.get_mut(selected) {
                item.completed = !item.completed;
            }
        }
    }

    pub fn get_list_to_display(&self) -> Vec<String> {
        self.items.iter().map(|item| {
            let status = if item.completed { "[✓]" } else { "[✗]" };
            format!("{} {} - {}", status, item.title, item.status.to_str())
        }).collect()
    }

    pub fn get_selected_item_display(&self) -> String {
        let selected = match self.state.selected() {
            Some(idx) => idx,
            None => return "No item selected".to_string(),
        };
        if let Some(item) = self.items.get(selected) {
            format!("title: {}, id: {} \n description: \n {} \n completed: \n {} \n status: \n {} ", 
                item.title.clone(),
                item.id.to_string(),
                item.description.clone(),
                item.completed.to_string(),
                item.status.to_str().to_string()
            )
        } else {
            "No item selected".to_string()
        }
    }
}
