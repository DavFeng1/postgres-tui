use super::table::Table;

#[derive(Debug, Clone, Default)]
pub struct Database {
    pub name: String,
    pub tables: Vec<Table>,
}

impl Database {
    pub fn new(name: String, tables: Vec<Table>) -> Database {
        Self { name, tables }
    }

    pub fn create_table(&mut self, table: Table) {
        self.tables.push(table)
    }
}
