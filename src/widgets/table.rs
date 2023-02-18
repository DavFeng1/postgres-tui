#[derive(Debug, Clone, Default)]
pub struct Table {
    columns: Vec<String>,
}

impl Table {
    pub fn new(columns: Vec<String>) -> Self {
        Self { columns }
    }
}
