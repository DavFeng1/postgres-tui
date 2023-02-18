use super::database_table::DatabaseTable;

#[derive(Debug, Clone, Default)]
pub struct Database {
    pub name: String,
    pub tables: Vec<DatabaseTable>,
}

impl Database {
    pub fn new(name: String, tables: Vec<DatabaseTable>) -> Database {
        Self { name, tables }
    }

    pub fn create_table(&mut self, table: DatabaseTable) {
        self.tables.push(table)
    }
}
