#[derive(Debug, Clone, Default)]
pub struct DatabaseTable {
    pub name: String,
    pub columns: Vec<String>,
    pub is_focused: bool,
}

impl DatabaseTable {
    pub fn new(name: String, columns: Vec<String>) -> Self {
        Self {
            name,
            columns,
            is_focused: false,
        }
    }

    pub fn set_columns(&mut self, columns: Vec<String>) {
        self.columns = columns;
    }
}
