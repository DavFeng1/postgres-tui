use std::collections::HashMap;


#[derive(Debug, Clone, Default)]
pub struct DatabaseState {
    pub database_list: Vec<String>,
    pub tables_map: HashMap<String, Vec<String>>,
    // Index of the current database
    pub selected_database: Option<usize>,
    // Index of the current table
    pub selected_table: Option<usize>,
}

impl DatabaseState {
    pub fn next(&mut self) {
        let i = match self.selected_database {
            Some(i) => {
                if i >= self.database_list.len() - 1 {
                    self.database_list.len() - 1
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.selected_database = Some(i)
    }

    pub fn prev(&mut self) {
        let i: usize = match self.selected_database {
            Some(i) => {
                if i <= 1 {
                    0
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.selected_database = Some(i)
    }

    pub fn with_database_list(database_list: Vec<String>) -> Self {
        Self {
            database_list,
            tables_map: HashMap::default(),
            selected_database: None,
            selected_table: None,
        }
    }
}

