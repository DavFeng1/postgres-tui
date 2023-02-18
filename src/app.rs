use crossterm::event::{self, Event, KeyCode};
use postgres::{Client, Error};
use std::io;

use crate::{
    postgres::{connect, query::get_databases},
    widgets::{
        database::Database, database_cluster::DatabaseCluster, database_table::DatabaseTable,
    },
};

#[derive(Debug, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

pub struct PSQLConnectionOptions {
    pub host: String,
    pub user: String,
    pub dbname: Option<String>,
}

#[derive(PartialEq, Eq)]
pub enum FocusElement {
    Sidebar,
    Main,
    SearchBar,
}

pub struct App {
    pub connection: Option<Client>,
    pub database_state: DatabaseCluster,
    pub debug_message: String,
    pub focused_element: FocusElement,
    pub input: String,
    pub input_mode: InputMode,
    pub selected_database: String,
    pub show_keybinds: bool,
    pub should_quit: bool,
    pub title: String,
    input_history: Vec<String>,
}

impl App {
    pub fn new(title: String) -> App {
        let default_connection_options = PSQLConnectionOptions {
            user: String::from("postgres"),
            host: String::from("localhost"),
            dbname: None,
        };

        let mut connection = connect(default_connection_options).expect("Postgres client");

        let databases = get_databases(&mut connection)
            .into_iter()
            .map(|row| Database {
                name: row.get(0),
                tables: Vec::new(),
            })
            .collect();

        App {
            database_state: DatabaseCluster::new(databases),
            title,
            should_quit: false,
            show_keybinds: true,
            input: String::new(),
            input_mode: InputMode::Normal,
            input_history: Vec::new(),
            debug_message: String::from("test"),
            connection: Some(connection),
            focused_element: FocusElement::Sidebar,
            selected_database: String::from(""),
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
                    KeyCode::Enter => self.handle_database_select(),
                    KeyCode::Char('j') => self.database_state.next(),
                    KeyCode::Char('k') => self.database_state.prev(),
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

    fn handle_database_select(&mut self) {
        self.database_state.toggle_select_focused_element();

        match self.database_state.selected_database {
            Some(selected_db_index) => {
                let selected_database_name: String = self.database_state.databases
                    [selected_db_index]
                    .name
                    .clone();

                self.update_connection(selected_database_name)
                    .expect("Could not update connection for newly selected database");
            }

            None => {}
        }
    }

    fn update_connection(&mut self, database_name: String) -> Result<(), Error> {
        {
            self.selected_database = database_name.clone();

            let connection_to_selected_database = PSQLConnectionOptions {
                host: String::from("localhost"),
                user: String::from("postgres"),
                dbname: Some(self.selected_database.clone()),
            };

            let mut connection = connect(connection_to_selected_database).expect("db client");
            let result = connection
                .query(
                    "SELECT tablename FROM pg_tables where schemaname = 'public'",
                    &[],
                )
                .expect("Could not get tables for database");

            let mut table_names = vec![];
            for row in result {
                let name: String = row.get(0);
                table_names.push(name);
            }

            for database in self.database_state.databases.iter_mut() {
                if database.name.clone() == database_name {
                    let tables_for_database = table_names
                        .into_iter()
                        .map(|name| DatabaseTable::new(name, Vec::new()))
                        .collect();

                    database.tables = tables_for_database;

                    break;
                }
            }

            self.connection = Some(connection);

            Ok(())
        }
    }
}
