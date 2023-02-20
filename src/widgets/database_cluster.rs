use std::collections::HashMap;

use super::database::Database;

// Store the state that the database tree can easily read and render
#[derive(Debug, Clone, Default)]
pub struct DatabaseCluster {
    pub databases: Vec<Database>,
    pub tables_map: HashMap<String, Vec<String>>,
    pub current_connected_database: Option<usize>,
    pub current_focused_database: Option<usize>,
}

impl DatabaseCluster {
    pub fn new(databases: Vec<Database>) -> Self {
        Self {
            databases,
            tables_map: HashMap::default(),
            current_connected_database: None,
            current_focused_database: None,
        }
    }

    pub fn next(&mut self) {
        let next_database_index = match self.current_focused_database {
            Some(focused_db_index) => {
                self.databases[focused_db_index].is_focused = false;
                let number_of_databases = self.databases.len();
                if focused_db_index >= number_of_databases - 1 {
                    0
                } else {
                    focused_db_index + 1
                }
            }
            None => 0,
        };

        self.current_focused_database = Some(next_database_index);
        self.databases[next_database_index].is_focused = true;
    }

    pub fn prev(&mut self) {
        let prev_database_index = match self.current_focused_database {
            Some(focused_db_index) => {
                self.databases[focused_db_index].is_focused = false;
                let number_of_databases = self.databases.len();
                if focused_db_index <= 0 {
                    number_of_databases - 1
                } else {
                    focused_db_index - 1
                }
            }
            None => 0,
        };

        self.current_focused_database = Some(prev_database_index);
        self.databases[prev_database_index].is_focused = true;
    }

    pub fn toggle_select_focused_element(&mut self) {
        match self.current_focused_database {
            Some(focused_db_index) => match self.current_connected_database {
                Some(connected_db_index) => {
                    if focused_db_index == connected_db_index {
                        self.databases[connected_db_index].is_connected = false;
                        self.current_connected_database = None
                    } else {
                        self.databases[connected_db_index].is_connected = false;
                        self.databases[focused_db_index].is_connected = true;
                        self.current_connected_database = Some(focused_db_index);
                    }
                }
                None => {
                    self.databases[focused_db_index].is_connected = true;
                    self.current_connected_database = Some(focused_db_index);
                }
            },
            None => (),
        }
    }
}
