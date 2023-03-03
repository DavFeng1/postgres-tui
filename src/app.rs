use crossterm::event::{self, Event, KeyCode, KeyEvent};
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
    Explorer,
    Main,
    SearchBar,
}

// App should store state which are separate from widgets.
// Widgets should read the state and determin what to render.
pub struct App {
    pub connection: Option<Client>,
    pub cluster: DatabaseCluster,
    pub debug_message: String,
    pub focused_element: FocusElement,
    pub input: String,
    pub input_mode: InputMode,
    pub show_keybinds: bool,
    pub should_quit: bool,
    pub title: String,
    input_history: Vec<String>,
}

impl App {
    pub fn new(title: String) -> App {
        let default_connection_options = PSQLConnectionOptions {
            user: String::from("dfeng"),
            host: String::from("localhost"),
            dbname: None,
        };

        let mut connection = connect(default_connection_options).expect("Postgres client");

        let mut databases: Vec<Database> = get_databases(&mut connection)
            .into_iter()
            .map(|row| Database::new(row.get(0), Vec::new()))
            .collect();

        databases.sort_by(|a, b| a.name.cmp(&b.name));

        App {
            cluster: DatabaseCluster::new(databases),
            title,
            should_quit: false,
            show_keybinds: true,
            input: String::new(),
            input_mode: InputMode::Normal,
            input_history: Vec::new(),
            debug_message: String::from("test"),
            connection: Some(connection),
            focused_element: FocusElement::Explorer,
        }
    }

    // Register keybinds each time the app is updated.
    // Keybinds react to state and the current focused element
    //
    // Precedence order
    // 1) Input mode
    // 2) Focused Element
    //
    pub fn register_keybinds(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            match self.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('1') => self.focused_element = FocusElement::Explorer,
                    KeyCode::Char('2') => self.focused_element = FocusElement::SearchBar,
                    KeyCode::Char('3') => self.focused_element = FocusElement::Main,
                    KeyCode::Char('q') => self.should_quit = true,
                    KeyCode::Char('b') => self.show_keybinds = !self.show_keybinds,
                    _ => match self.focused_element {
                        FocusElement::Main => self.register_main_keybinds(key),
                        FocusElement::Explorer => self.register_explorer_keybinds(key),
                        FocusElement::SearchBar => self.register_searchbar_keybinds(key),
                    },
                },
                InputMode::Editing => self.register_edit_mode_keybinds(key),
            }
        }

        Ok(())
    }

    fn register_edit_mode_keybinds(&mut self, key: KeyEvent) {
        match key.code {
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

    fn register_explorer_keybinds(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => self.handle_select(),
            KeyCode::Char('j') => self.cluster.next(),
            KeyCode::Char('k') => self.cluster.prev(),
            KeyCode::Char('o') => self.open_table(),
            _ => {}
        }
    }

    fn register_main_keybinds(&mut self, key: KeyEvent) {
        match key.code {
            _ => {}
        }
    }

    fn register_searchbar_keybinds(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('i') => self.input_mode = InputMode::Editing,
            KeyCode::Esc => {
                self.input_mode = InputMode::Normal;
            }
            _ => {}
        }
    }

    fn open_table(&mut self) {
        print!("OPen sesame");
    }

    fn handle_select(&mut self) {
        self.cluster.toggle_select_focused_element();

        for database in self.cluster.databases.iter_mut() {
            if database.is_connected {
                let database_name = database.name.clone();

                self.update_connection(database_name)
                    .expect("Could not update connecion for newly selected database");

                break;
            }
        }
    }

    fn update_connection(&mut self, database_name: String) -> Result<(), Error> {
        {
            let connection_to_selected_database = PSQLConnectionOptions {
                host: String::from("localhost"),
                user: String::from("dfeng"),
                dbname: Some(database_name.clone()),
            };

            let mut connection = connect(connection_to_selected_database).expect("db client");
            let result = connection
                .query(
                    "SELECT tablename FROM pg_tables where schemaname = 'public'",
                    &[],
                )
                .expect("Could not get tables for database");

            let mut table_names: Vec<String> = result.iter().map(|row| row.get(0)).collect();

            table_names.sort();

            for database in self.cluster.databases.iter_mut() {
                if database.name == database_name {
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
