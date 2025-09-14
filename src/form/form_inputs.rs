use ratatui::crossterm::event::KeyCode;

pub mod string_field;
pub mod enum_field;

pub trait FormInputWidget {
    fn on_key_press(&mut self, key: KeyCode);
}
