
use ratatui::{crossterm::event::{KeyCode, KeyEvent}, layout::{Constraint, Layout, Offset, Rect}, Frame};
use serde::Serialize;
use uuid::Uuid;

use crate::{form::{form_inputs::{enum_field::EnumField, string_field::StringField, FormInputWidget}, form_status::FormStatus}, todo::TodoStatus};


#[derive(Serialize, Default, PartialEq, Eq, Debug)]
pub enum TodoFormState {
    Id,
    #[default]
    Title,
    Description,
    Status,
}

#[derive(Serialize, Debug)]
pub struct TodoForm {
    pub form_status: FormStatus,
    pub form_render_state: TodoFormState,
    pub id: StringField,
    pub title: StringField,
    pub description: StringField,
    #[serde(skip)]
    pub status: EnumField<TodoStatus>,
}


impl Default for TodoForm {
    fn default() -> Self {
        Self {
            form_status: FormStatus::default(),
            form_render_state: TodoFormState::default(),
            id: StringField::new("ID").set_default_value(Uuid::new_v4().to_string()),
            title: StringField::new("Title"),
            description: StringField::new("Description"),
            status: EnumField::new("Status", &[TodoStatus::Pending, TodoStatus::InProgress, TodoStatus::Completed]),
        }
    }
}

impl TodoForm {

    pub fn reset(&mut self) {
        self.clear();
        self.id.set_value(Uuid::new_v4().to_string());
    }

    pub fn on_key_press(&mut self, key: KeyCode, key_event: KeyEvent) {
        let key = self.form_status.handle_key_press(key, key_event);
        if key.is_none() {
            return;
        }
        let key = key.unwrap();
        match key {
            KeyCode::Tab => {
                self.next_field();
                return;
            }
            KeyCode::BackTab => {
                self.previous_field();
                return;
            }
            _ => {
                self.current_field_mut().on_key_press(key);
            }
        }
    }

    pub fn render(&self, area: Rect, frame: &mut Frame) {
        let [id_area, title_area, des_area, status_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .margin(1)
        .areas(area);

        frame.render_widget(&self.id, id_area);
        frame.render_widget(&self.title, title_area);
        frame.render_widget(&self.description, des_area);
        frame.render_widget(&self.status, status_area);

        let cursor_position = match self.form_render_state {
            TodoFormState::Id => id_area.offset(self.id.cursor_offset()),
            TodoFormState::Title => title_area.offset(self.title.cursor_offset()),
            TodoFormState::Description => des_area.offset(self.description.cursor_offset()),
            TodoFormState::Status => status_area.offset(Offset { x: 0, y: 0 }),
        };
        frame.set_cursor_position(cursor_position);
    }

    fn current_field_mut(&mut self) -> &mut dyn FormInputWidget {
        match self.form_render_state {
            TodoFormState::Id => &mut self.id,
            TodoFormState::Title => &mut self.title,
            TodoFormState::Description => &mut self.description,
            TodoFormState::Status => &mut self.status,
        }
    }

    fn _current_field(&self) ->  &dyn FormInputWidget {
        match self.form_render_state {
            TodoFormState::Id => &self.id,
            TodoFormState::Title => &self.title,
            TodoFormState::Description => &self.description,
            TodoFormState::Status => &self.status,
        }
    }

    pub fn next_field(&mut self) {
        self.form_render_state = match self.form_render_state {
            TodoFormState::Id => TodoFormState::Title,
            TodoFormState::Title => TodoFormState::Description,
            TodoFormState::Description => TodoFormState::Status,
            TodoFormState::Status => TodoFormState::Id,
        }
    }

    pub fn previous_field(&mut self) {
        self.form_render_state = match self.form_render_state {
            TodoFormState::Id => TodoFormState::Status,
            TodoFormState::Title => TodoFormState::Id,
            TodoFormState::Description => TodoFormState::Title,
            TodoFormState::Status => TodoFormState::Description,
        }
    }

    pub fn clear(&mut self) {
        self.id.clear();
        self.title.clear();
        self.description.clear();
        // self.status.clear();
    }

    pub fn _is_complete(&self) -> bool {
        !self.title.is_empty() && !self.description.is_empty()
    }
}
