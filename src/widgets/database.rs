use super::database_table::DatabaseTable;

#[derive(Debug, Clone, Default)]
pub struct Database {
    pub name: String,
    pub tables: Vec<DatabaseTable>,
    pub is_focused: bool,
    pub is_connected: bool,
}

impl Database {
    pub fn new(name: String, tables: Vec<DatabaseTable>) -> Database {
        Self {
            name,
            tables,
            is_focused: false,
            is_connected: false,
        }
    }

    pub fn create_table(&mut self, table: DatabaseTable) {
        self.tables.push(table)
    }
}
