
use ratatui::crossterm::event::KeyCode;
use serde::Serialize;


#[derive(Default, Clone, PartialEq, Eq, Serialize, Debug)]
pub enum FormStatus {
    #[default]
    Viewing,
    Editing,
    // Creating,
}

impl FormStatus {

    // pub fn handle_key_press<F>(&mut self, key_event: KeyEvent, mut next: F)
    // where F: FnMut(KeyEvent), {
    //     if let event::KeyEventKind::Press = key_event.kind {
    //         match self {
    //             FormStatus::Viewing => todo!(),
    //             FormStatus::Editing => {
    //                 if key_event.code == KeyCode::Esc {
    //                     *self = FormStatus::Viewing;
    //                     return;
    //                 }
    //                 next(key_event);
    //             },
    //         }
    //     }
    // }

    pub fn handle_key_press(&mut self, key: KeyCode) -> Option<KeyCode> {
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
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            FormStatus::Viewing => "Viewing",
            FormStatus::Editing => "Editing",
        }
    }

    pub fn _is_viewing(&self) -> bool {
        matches!(self, FormStatus::Viewing)
    }

    pub fn is_editing(&self) -> bool {
        matches!(self, FormStatus::Editing)
    }
}
