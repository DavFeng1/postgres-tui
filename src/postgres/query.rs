use postgres::{Client, Row};

pub fn get_databases(client: &mut Client) -> Vec<Row> {
    client
        .query(
            "SELECT datname from pg_database WHERE datistemplate = false",
            &[],
        )
        .expect("Get databases")
}
