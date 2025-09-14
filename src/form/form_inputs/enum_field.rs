use ratatui::{
    buffer::Buffer, crossterm::event::KeyCode, layout::{Constraint, Layout, Rect}, text::Line, widgets::Widget
};
use serde::Serialize;

use crate::form::form_inputs::FormInputWidget;

// Trait to convert enum variants into &str for display
pub trait EnumDisplay {
    fn to_str(&self) -> &str;
}

#[derive(Default, Clone, Serialize, Debug, Copy)]
pub struct EnumField<T>
where
    T: EnumDisplay + Copy + PartialEq + 'static,
{
    label: &'static str,
    options: &'static [T],
    selected_index: usize,
}

impl<T> FormInputWidget for EnumField<T> where T: EnumDisplay + Copy + PartialEq + 'static {
    fn on_key_press(&mut self, key: KeyCode) {
        match key {
            KeyCode::Left | KeyCode::Up => {
                if self.selected_index == 0 {
                    self.selected_index = self.options.len() - 1;
                } else {
                    self.selected_index -= 1;
                }
            }
            KeyCode::Right | KeyCode::Down => {
                self.selected_index = (self.selected_index + 1) % self.options.len();
            }
            _ => {}
        }
    }
}

impl<T> EnumField<T>
where
    T: EnumDisplay + Copy + PartialEq + 'static,
{
    pub fn new(label: &'static str, options: &'static [T]) -> Self {
        Self {
            label,
            options,
            selected_index: 0,
        }
    }

    pub fn get_value(&self) -> T {
        self.options[self.selected_index]
    }

    pub fn set_value(&mut self, value: T) {
        if let Some(pos) = self.options.iter().position(|&v| v == value) {
            self.selected_index = pos;
        }
    }

}

impl<T> Widget for &EnumField<T>
where
    T: EnumDisplay + Copy + PartialEq + 'static,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [label_area, value_area] = Layout::horizontal([
            Constraint::Length(self.label.len() as u16 + 2),
            Constraint::Fill(1),
        ])
        .areas(area);

        let label = Line::from_iter([self.label, ": "]);
        label.render(label_area, buf);

        let value_str = self.get_value();
        Line::from(value_str.to_str()).render(value_area, buf);
    }
}
