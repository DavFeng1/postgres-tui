use std::collections::HashMap;

use super::database::Database;

#[derive(Debug, Clone, Default)]
pub struct DatabaseCluster {
    pub databases: Vec<Database>,
    pub tables_map: HashMap<String, Vec<String>>,
    pub focused_element: Option<usize>,
    // Index of the current database
    pub selected_database: Option<usize>,
    // Index of the current table
    pub selected_table: Option<usize>,
}

impl DatabaseCluster {
    pub fn next(&mut self) {
        let i = match self.focused_element {
            Some(i) => {
                if i >= self.databases.len() - 1 {
                    self.databases.len() - 1
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.focused_element = Some(i)
    }

    pub fn prev(&mut self) {
        let i: usize = match self.focused_element {
            Some(i) => {
                if i <= 1 {
                    0
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.focused_element = Some(i)
    }

    pub fn toggle_select_focused_element(&mut self) {
        match self.focused_element {
            Some(focused_element) => match self.selected_database {
                Some(selected_database) => {
                    if focused_element == selected_database {
                        self.selected_database = None;
                    } else {
                        self.selected_database = Some(focused_element);
                    }
                }

                None => {
                    self.selected_database = Some(focused_element);
                }
            },
            None => {}
        }
    }

    pub fn new(databases: Vec<Database>) -> Self {
        Self {
            databases,
            tables_map: HashMap::default(),
            focused_element: None,
            selected_database: None,
            selected_table: None,
        }
    }
}
