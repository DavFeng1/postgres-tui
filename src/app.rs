use crossterm::event::{self, Event, KeyCode};
use postgres::Client;
use tui::widgets::ListState;

use std::io;

use crate::postgres::{connect, query::get_databases};

#[derive(Debug, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

pub struct PSQLConnectionOptions {
    pub host: String,
    pub user: String,
}

#[derive(PartialEq, Eq)]
pub enum FocusElement {
    Sidebar,
    Main,
    SearchBar,
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

pub struct DatabaseState {
    pub items: StatefulList<String>,
}

pub struct App {
    pub title: String,
    pub show_keybinds: bool,
    pub should_quit: bool,
    pub input: String,
    pub input_mode: InputMode,
    pub debug_message: String,
    pub postgres_client: Client,
    pub focused_element: FocusElement,
    pub database_state: DatabaseState,
    pub selected_database: String,
    input_history: Vec<String>,
}

impl App {
    pub fn new(title: String) -> App {
        let default_connection_options = PSQLConnectionOptions {
            host: String::from("localhost"),
            user: String::from("postgres"),
        };

        let mut client = connect(default_connection_options).expect("Postgres client");

        let mut database_list = vec![];

        for row in get_databases(&mut client) {
            let database_name: String = row.get(0);
            database_list.push(database_name);
        }

        let database_state = DatabaseState {
            items: StatefulList::with_items(database_list),
        };

        App {
            title,
            should_quit: false,
            show_keybinds: true,
            input: String::new(),
            input_mode: InputMode::Normal,
            input_history: Vec::new(),
            debug_message: String::from("test"),
            postgres_client: client,
            focused_element: FocusElement::Sidebar,
            selected_database: String::from(""),
            database_state,
        }
    }

    pub fn register_keybinds(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            match self.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('1') => self.focused_element = FocusElement::Sidebar,
                    KeyCode::Char('2') => self.focused_element = FocusElement::SearchBar,
                    KeyCode::Char('3') => self.focused_element = FocusElement::Main,
                    KeyCode::Char('i') => {
                        self.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        self.should_quit = true;
                    }
                    KeyCode::Char('b') => {
                        self.show_keybinds = !self.show_keybinds;
                    }
                    KeyCode::Enter => self.database_state.items.unselect(),
                    KeyCode::Char('j') => self.database_state.items.next(),
                    KeyCode::Char('k') => self.database_state.items.previous(),
                    _ => {}
                },

                InputMode::Editing => match self.focused_element {
                    FocusElement::SearchBar => match key.code {
                        KeyCode::Enter => {
                            self.input_history.push(self.input.drain(..).collect());
                        }
                        KeyCode::Char(c) => {
                            self.input.push(c);
                        }
                        KeyCode::Backspace => {
                            self.input.pop();
                        }
                        KeyCode::Esc => {
                            self.input_mode = InputMode::Normal;
                        }
                        _ => {}
                    },
                    _ => match key.code {
                        KeyCode::Esc => {
                            self.input_mode = InputMode::Normal;
                        }
                        _ => {}
                    },
                },
            }
        }

        Ok(())
    }
}
