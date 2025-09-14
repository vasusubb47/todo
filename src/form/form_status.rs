
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::Serialize;


#[derive(Default, Clone, PartialEq, Eq, Serialize, Debug)]
pub enum FormStatus {
    #[default]
    Viewing,
    Editing,
    Submitting,
    // Creating,
}

impl FormStatus {

    pub fn handle_key_press(&mut self, key: KeyCode, key_event: KeyEvent) -> Option<KeyCode> {
        if key_event.modifiers.contains(KeyModifiers::CONTROL) {
            match key {
                KeyCode::Char('s') => {
                    *self = FormStatus::Submitting;
                    return None;
                }
                _ => {}
            }
            return None;
        }
        match self {
            FormStatus::Viewing => {
                match key {
                    KeyCode::Enter => {
                        *self = FormStatus::Editing;
                        return None;
                    }
                    _ => {return None;}
                }
            },
            FormStatus::Editing => {
                match key {
                    KeyCode::Enter => {
                        *self = FormStatus::Viewing;
                        return None;
                    }
                    _ => {return Some(key);}
                }
            },
            FormStatus::Submitting => {
                return None;
            },
        }
    }

    pub fn reset(&mut self) {
        *self = FormStatus::Viewing;
    }

    pub fn to_str(&self) -> &str {
        match self {
            FormStatus::Viewing => "Viewing",
            FormStatus::Editing => "Editing",
            FormStatus::Submitting => "Submitting",
        }
    }

    pub fn _is_viewing(&self) -> bool {
        matches!(self, FormStatus::Viewing)
    }

    pub fn is_editing(&self) -> bool {
        matches!(self, FormStatus::Editing)
    }

    pub fn is_submitting(&self) -> bool {
        matches!(self, FormStatus::Submitting)
    }
}
