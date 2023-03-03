use std::collections::HashMap;

use super::database::Database;

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
        match self.current_connected_database {
            Some(current_connnected_database) => {
                let next_table_index = match self.current_focused_table {
                    Some(focused_table_index) => {
                        self.databases[current_connnected_database].tables[focused_table_index]
                            .is_focused = false;

                        let number_of_tables =
                            self.databases[current_connnected_database].tables.len();

                        if focused_table_index >= number_of_tables - 1 {
                            0
                        } else {
                            focused_table_index + 1
                        }
                    }
                    None => 0,
                };

                self.current_focused_table = Some(next_table_index);
                self.databases[current_connnected_database].tables[next_table_index].is_focused =
                    true;
            }
            None => (),
        }
    }

    pub fn prev_table(&mut self) {
        match self.current_connected_database {
            Some(current_connnected_database) => {
                let prev_table_index = match self.current_focused_table {
                    Some(focused_table_index) => {
                        self.databases[current_connnected_database].tables[focused_table_index]
                            .is_focused = false;

                        let number_of_tables =
                            self.databases[current_connnected_database].tables.len();

                        if focused_table_index <= 0 {
                            number_of_tables - 1
                        } else {
                            focused_table_index - 1
                        }
                    }
                    None => 0,
                };

                self.current_focused_table = Some(prev_table_index);
                self.databases[current_connnected_database].tables[prev_table_index].is_focused =
                    true;
            }
            None => (),
        }
    }

    pub fn toggle_select_focused_element(&mut self) {
        match self.current_focused_database {
            Some(focused_db_index) => {
                match self.current_connected_database {
                    Some(connected_db_index) => {
                        if focused_db_index == connected_db_index {
                            self.databases[connected_db_index].is_connected = false;
                            self.current_connected_database = None
                        } else {
                            self.databases[connected_db_index].is_connected = false;
                            self.databases[focused_db_index].is_connected = true;
                            self.current_connected_database = Some(focused_db_index);
                        };

                        match self.current_focused_table {
                            Some(current_focused_table) => {
                                self.databases[connected_db_index].tables[current_focused_table]
                                    .is_focused = false;
                                self.current_focused_table = None;
                            }
                            None => (),
                        }
                    }
                    None => {
                        self.databases[focused_db_index].is_connected = true;
                        self.current_connected_database = Some(focused_db_index);
                    }
                };
            }
            None => (),
        }
    }
}
