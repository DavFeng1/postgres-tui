use postgres::Client;
use crossterm::event::{self, KeyCode, Event};
use std::io;

use crate::postgres::connect;

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

pub struct App {
    pub title: String,
    pub show_keybinds: bool,
    pub should_quit: bool,
    pub input: String,
    pub input_mode: InputMode,
    pub debug_message: String,
    pub postgres_client: Client,
    pub focused_element: FocusElement,
    input_history: Vec<String>,
}


impl App {

    pub fn new(title: String) -> App {
        let default_connection_options = PSQLConnectionOptions {
            host:  String::from("localhost"),
            user: String::from("postgres"),
        };

        let client = connect(default_connection_options)
            .expect("Grabbing postgres client");

        App {
            title,
            should_quit: false,
            show_keybinds: true,
            input: String::new(),
            input_mode: InputMode::Normal,
            input_history: Vec::new(),
            debug_message: String::from("test"),
            postgres_client: client,
            focused_element: FocusElement::Sidebar
        }
    }

    pub fn register_keybinds(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            match self.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('1') => {
                        self.focused_element = FocusElement::Sidebar
                    }
                    KeyCode::Char('2') => {
                        self.focused_element = FocusElement::SearchBar
                    }
                    KeyCode::Char('3') => {
                        self.focused_element = FocusElement::Main
                    }
                    KeyCode::Char('e') => {
                        self.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        self.should_quit = true;
                    },
                    KeyCode::Char('k') => {
                        self.show_keybinds = !self.show_keybinds;
                    }
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
                    }
                    _ => match key.code {
                        KeyCode::Esc => {
                            self.input_mode = InputMode::Normal;
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }
}
