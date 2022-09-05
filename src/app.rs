use crossterm::event::{self, KeyCode, Event};
use std::io;

#[derive(Debug)]
pub enum InputMode {
    Normal,
    Editing,
}


pub struct App<'a> {
    pub title: &'a str,
    pub show_popup: bool,
    pub should_quit: bool,
    pub input: String,
    pub input_mode: InputMode,
    input_history: Vec<String>,
}


impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            title,
            should_quit: false,
            show_popup: true,
            input: String::new(),
            input_mode: InputMode::Normal,
            input_history: Vec::new(),
        }
    }

    pub fn register_keybinds(&mut self) -> io::Result<()> {

        if let Event::Key(key) = event::read()? {
            match self.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        self.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        self.should_quit = true;
                    },
                    KeyCode::Char('p') => {
                        self.show_popup = !self.show_popup;
                    }
                    _ => {}
                },

                InputMode::Editing => match key.code {
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
            }
        }

        return Ok(());
    }
}
