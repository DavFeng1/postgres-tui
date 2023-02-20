use std::collections::HashMap;

use super::database::Database;

// Store the state that the database tree can easily read and render
#[derive(Debug, Clone, Default)]
pub struct DatabaseCluster {
    pub databases: Vec<Database>,
    pub tables_map: HashMap<String, Vec<String>>,
}

impl DatabaseCluster {
    pub fn new(databases: Vec<Database>) -> Self {
        Self {
            databases,
            tables_map: HashMap::default(),
        }
    }

    pub fn next(&mut self) {
        let mut is_next = false;
        for database in self.databases.iter_mut() {
            if is_next {
                database.is_focused = true;
                break;
            };

            if database.is_focused {
                database.is_focused = !database.is_focused;
                is_next = true;

                continue;
            };
        }

        if !is_next {
            match self.databases.first_mut() {
                Some(first_database) => first_database.is_focused = true,
                None => {}
            }
        }
    }

    pub fn prev(&mut self) {
        let mut is_prev = false;
        for database in self.databases.iter_mut().rev() {
            if is_prev {
                database.is_focused = true;

                break;
            };

            if database.is_focused {
                database.is_focused = !database.is_focused;
                is_prev = true;
            };
        }

        if !is_prev {
            match self.databases.last_mut() {
                Some(last_database) => last_database.is_focused = true,
                None => {}
            }
        }
    }

    pub fn toggle_select_focused_element(&mut self) {
        for database in self.databases.iter_mut() {
            if database.is_focused {
                database.is_connected = !database.is_connected;
                break;
            }
        }
    }
}
