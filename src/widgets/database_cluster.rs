use std::collections::HashMap;

use super::{database::Database, database_table::DatabaseTable};

pub enum TreeElement {
    Database,
    Table,
}

pub struct DatabaseCluster {
    pub databases: Vec<Database>,
    pub tables_map: HashMap<String, Vec<String>>,
    pub current_connected_database: Option<usize>,
    pub current_selected_table: Option<usize>,
    pub current_focused_database: Option<usize>,
    pub current_focused_table: Option<usize>,
    pub current_focused_element: Option<TreeElement>,
}

impl DatabaseCluster {
    pub fn new(databases: Vec<Database>) -> Self {
        Self {
            databases,
            tables_map: HashMap::default(),
            current_connected_database: None,
            current_selected_table: None,
            current_focused_database: None,
            current_focused_table: None,
            current_focused_element: None,
        }
    }

    pub fn next(&mut self) {
        match self.current_connected_database {
            Some(_current_connected_database) => self.next_table(),
            None => self.next_database(),
        };
    }

    pub fn prev(&mut self) {
        match self.current_connected_database {
            Some(_current_connected_database) => self.prev_table(),
            None => self.prev_database(),
        };
    }

    pub fn next_database(&mut self) {
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

    pub fn prev_database(&mut self) {
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

    pub fn next_table(&mut self) {
        let connected_database_index = match self.current_connected_database {
            Some(connected_database_index) => connected_database_index,
            None => return (),
        };

        let current_database = &mut self.databases[connected_database_index];
        if current_database.tables.len() == 0 {
            return ();
        };

        let next_table_index = match self.current_focused_table {
            Some(focused_table_index) => {
                current_database.tables[focused_table_index].is_focused = false;

                let number_of_tables = current_database.tables.len();

                if focused_table_index >= number_of_tables - 1 {
                    0
                } else {
                    focused_table_index + 1
                }
            }
            None => 0,
        };

        self.current_focused_table = Some(next_table_index);
        current_database.tables[next_table_index].is_focused = true;
    }

    pub fn prev_table(&mut self) {
        let connected_database_index = match self.current_connected_database {
            Some(connected_database_index) => connected_database_index,
            None => return (),
        };

        let current_database = &mut self.databases[connected_database_index];
        if current_database.tables.len() == 0 {
            return ();
        };

        let prev_table_index = match self.current_focused_table {
            Some(focused_table_index) => {
                current_database.tables[focused_table_index].is_focused = false;

                let number_of_tables = current_database.tables.len();

                if focused_table_index <= 0 {
                    number_of_tables - 1
                } else {
                    focused_table_index - 1
                }
            }
            // This panics when there are no tables in the database
            None => 0,
        };

        self.current_focused_table = Some(prev_table_index);
        current_database.tables[prev_table_index].is_focused = true;
    }

    pub fn toggle_focused_database(&mut self) {
        self.current_selected_table = None;
        self.current_focused_table = None;

        let focused_database_index = match self.current_focused_database {
            Some(focused_database_index) => focused_database_index,
            None => return (),
        };

        match self.current_connected_database {
            Some(connected_database_index) => {
                if focused_database_index == connected_database_index {
                    self.databases[connected_database_index].is_connected = false;
                    self.current_connected_database = None;
                } else {
                    self.databases[focused_database_index].is_connected = true;
                    self.databases[connected_database_index].is_connected = false;
                    self.current_connected_database = Some(focused_database_index);
                }
            }
            None => {
                self.databases[focused_database_index].is_connected = true;
                self.current_connected_database = Some(focused_database_index);
            }
        };
    }

    pub fn select_focused_table(&mut self) -> Option<&mut DatabaseTable> {
        match self.current_connected_database {
            Some(current_db_index) => {
                let current_database = &mut self.databases[current_db_index];

                match self.current_focused_table {
                    Some(current_table_index) => {
                        let current_table = &mut current_database.tables[current_table_index];
                        self.current_selected_table = Some(current_table_index);
                        Some(current_table)
                    }
                    None => None,
                }
            }
            None => None,
        }
    }

    pub fn get_current_selected_table(&self) -> Option<DatabaseTable> {
        match self.current_connected_database {
            Some(current_db) => {
                let current_database = &self.databases[current_db];
                match self.current_selected_table {
                    Some(current_table_index) => {
                        let current_database = current_database.tables[current_table_index].clone();
                        Some(current_database)
                    }
                    None => None,
                }
            }
            None => None,
        }
    }
}
