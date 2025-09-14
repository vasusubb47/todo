use ratatui::{
    buffer::Buffer,
    crossterm::event::KeyCode,
    layout::{Constraint, Layout, Offset, Rect},
    text::Line,
    widgets::Widget,
};
use serde::Serialize;

use crate::form::form_inputs::FormInputWidget;

#[derive(Default, Clone, Serialize, Debug)]
pub struct StringField {
    label: &'static str,
    value: String,
}

impl FormInputWidget for StringField {
    /// Handle input events for the string input.
    fn on_key_press(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) => self.value.push(c),
            KeyCode::Backspace => {
                self.value.pop();
            }
            _ => {}
        }
    }
}

impl StringField {
    pub fn new(label: &'static str) -> Self {
        Self {
            label,
            value: String::new(),
        }
    }

    pub fn set_default_value(&self, value: String) -> Self {
        Self {
            label: self.label,
            value,
        }
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    pub fn cursor_offset(&self) -> Offset {
        let x = (self.label.len() + self.value.len() + 2) as i32;
        Offset { x, y: 0 }
    }

    pub fn clear(&mut self) {
        self.value.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

impl Widget for &StringField {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [label_area, value_area] = Layout::horizontal([
            Constraint::Length(self.label.len() as u16 + 2),
            Constraint::Fill(1),
        ])
        .areas(area);
        let label = Line::from_iter([self.label, ": "]);
        label.render(label_area, buf);
        self.value.clone().render(value_area, buf);
    }
}
