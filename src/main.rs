use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode}, layout::{Constraint, Layout}, style::{Color, Style}, text::ToSpan, widgets::{Block, BorderType, List, ListState, Paragraph, StatefulWidget, Widget}, DefaultTerminal, Frame
};
use std::fs;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
enum AppMode {
    Normal = 0,
    Editing,
    Adding,
}

#[derive(Debug, Deserialize, Serialize)]
enum TodoStatus {
    Completed,
    InProgress,
    Pending,
}

impl TodoStatus{
    fn to_str(&self) -> &str {
        match self {
            TodoStatus::Completed => "Completed",
            TodoStatus::InProgress => "InProgress",
            TodoStatus::Pending => "Pending",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct TodoItem {
    id: Uuid,
    title: String,
    description: String,
    completed: bool,
    status: TodoStatus,
}

#[derive(Debug)]
struct AppState {
    todos: TodoList,
    mode: AppMode
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            todos: TodoList::default(),
            mode: AppMode::Normal,
        }
    }
}

#[derive(Debug, Default)]
struct TodoList {
    items: Vec<TodoItem>,
    state: ListState,
}

impl TodoList {
    fn read_todos(&mut self) {
        let path = ".\\data\\data.json";
        let data = fs::read_to_string(path).expect("Unable to read file");
        let todos: Vec<TodoItem> = serde_json::from_str(&data).expect("JSON was not well-formatted");
        
        self.items = todos;
        self.state = ListState::default();
    }

    fn save_todos(&self) {
        let path = ".\\data\\data.json";
        let data = serde_json::to_string(&self.items).expect("Unable to serialize data");
        fs::write(path, data).expect("Unable to write file");
    }

    fn remove_selected(&mut self) {
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

    fn mark_completed(&mut self) {
        if let Some(selected) = self.state.selected() {
            if let Some(item) = self.items.get_mut(selected) {
                item.completed = !item.completed;
            }
        }
    }
}


fn main() -> Result<()> {
    color_eyre::install()?;

    let terminal = ratatui::init();
    let mut app_state = AppState::default();
    // app_state.todos.test_todos();
    app_state.todos.read_todos();

    let result = run_app(terminal, &mut app_state);
    ratatui::restore();
    result
}

fn run_app(mut terminal: DefaultTerminal, mut app_state: &mut AppState) -> Result<()> {
    loop {
        terminal.draw(|frame| draw(frame, &mut app_state))?;


        if let Event::Key(key) = event::read()? {
            match key.kind {
                event::KeyEventKind::Press => {
                    match app_state.mode {
                        AppMode::Normal => {
                            if handle_normal_mode_input(key.code, &mut app_state) {
                                break;
                            }
                        },
                        AppMode::Editing => {
                            if handle_editing_mode_input(key.code, &mut app_state) {
                                break;
                            }
                        },
                        AppMode::Adding => {
                            if handle_adding_mode_input(key.code, &mut app_state) {
                                break;
                            }
                        },
                    }
                },
                _ => {}
            }
        }
    }

    Ok(())
}

fn handle_normal_mode_input(key: KeyCode, app_state: &mut AppState) -> bool {
    match key {
        KeyCode::Char('q') => {
            // Quit the application
            app_state.todos.save_todos();
            return true;
        }
        KeyCode::Char('h') => {
            app_state.todos.state.select(None);
        }
        KeyCode::Char('j') => {
            app_state.todos.state.select_previous();
        }
        KeyCode::Char('k') => {
            app_state.todos.state.select_next();
        }
        KeyCode::Char('l') => {
            app_state.todos.state.select_first();
        }
        KeyCode::Char('L') => {
            app_state.todos.state.select_last();
        }
        KeyCode::Char('D') => {
            app_state.todos.mark_completed();
        }
        KeyCode::Char('R') => {
            app_state.todos.remove_selected();
        }
        KeyCode::Char('a') => {
            app_state.mode = AppMode::Adding;
        }
        KeyCode::Char('e') => {
            app_state.mode = AppMode::Editing;
        }
        _ => {}
    }
    false
}

fn handle_adding_mode_input(key: KeyCode, app_state: &mut AppState) -> bool {
    match key {
        KeyCode::Esc => {
            app_state.mode = AppMode::Normal;
        }
        _ => {}
    }
    false
}

fn handle_editing_mode_input(key: KeyCode, app_state: &mut AppState) -> bool {
    match key {
        KeyCode::Esc => {
            app_state.mode = AppMode::Normal;
        }
        _ => {}
    }
    false
}

fn draw(frame: &mut Frame, app_state: &mut AppState) {
    let [main_layout] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());

    let [list_box, tips] = Layout::vertical([Constraint::Fill(1), Constraint::Length(3)])
        .margin(1)
        .areas(main_layout);

    let [todo_list_area, read_area] = Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)])
        .margin(1)
        .areas(list_box);

    Block::bordered()
        .border_type(BorderType::Rounded)
        .style(Style::new().bg(Color::Magenta))
        .render(main_layout, frame.buffer_mut());


    let items: Vec<String> = app_state
                        .todos
                        .items
                        .iter()
                        .map(|item| {
                            if item.completed {
                                format!("{} ✔", item.title)
                            } else {
                                format!("{} ✘", item.title)
                            }
                        })
                        .collect();
    
    let toto_list = List::new(items)
            .block(
                Block::bordered()
                    .title("List".to_span().into_centered_line())
                    .border_type(BorderType::Rounded)
            )
            .highlight_symbol(">");

    StatefulWidget::render(toto_list, todo_list_area, frame.buffer_mut(), &mut app_state.todos.state);

    frame.render_widget(
        Paragraph::new(format!("title: {}, id: {} \n description: \n {} \n completed: \n {} \n status: \n {} ", 
            app_state.todos.state.selected().map_or("None".to_string(), |i| app_state.todos.items[i].title.clone()),
            app_state.todos.state.selected().map_or("None".to_string(), |i| app_state.todos.items[i].id.to_string()),
            app_state.todos.state.selected().map_or("None".to_string(), |i| app_state.todos.items[i].description.clone()),
            app_state.todos.state.selected().map_or("None".to_string(), |i| app_state.todos.items[i].completed.to_string()),
            app_state.todos.state.selected().map_or("None".to_string(), |i| app_state.todos.items[i].status.to_str().to_string())
        ))
            .block(
                Block::bordered()
                    .title("Read".to_span().into_centered_line())
                    .border_type(BorderType::Rounded)
            ),
         read_area
    );

    match app_state.mode {
        AppMode::Normal => {
            frame.render_widget(
                // q -> quit
                // h -> deselect
                // j -> select previous
                // k -> select next
                // l -> select first
                // L -> select last
                // a -> add item
                Paragraph::new("q -> quit | h -> deselect | j -> select previous | k -> select next | l -> select first | L -> select last | a -> add item")
                    .block(
                        Block::bordered()
                            .title("Help".to_span().into_centered_line())
                            .border_type(BorderType::Rounded)
                    ),
                tips
            );
        },
        AppMode::Editing => {
            frame.render_widget(
                Paragraph::new("Editing mode not implemented yet.")
                    .block(
                        Block::bordered()
                            .title("Editing".to_span().into_centered_line())
                            .style(Style::new().bg(Color::Red))
                            .border_type(BorderType::Rounded)
                    ),
                tips
            );
            return;
        },
        AppMode::Adding => {
            frame.render_widget(
                Paragraph::new("Adding mode not implemented yet.")
                    .block(
                        Block::bordered()
                            .title("Adding".to_span().into_centered_line())
                            .style(Style::new().bg(Color::Red))
                            .border_type(BorderType::Rounded)
                    ),
                tips
            );
            return;
        },
        
    }
}
