use postgres::{Client, Error, NoTls, Row};

use crate::app::PSQLConnectionOptions;

pub struct ConnectionManager {
    client: Client,
    connection_options: PSQLConnectionOptions,
}

impl ConnectionManager {
    pub fn new(connection_options: PSQLConnectionOptions) -> Result<ConnectionManager, Error> {
        let client_result = Client::connect(
            format!(
                "host={} user={} dbname={}",
                connection_options.host, connection_options.user, connection_options.db_name,
            )
            .as_str(),
            NoTls,
        );

        match client_result {
            Ok(client) => Ok(ConnectionManager {
                client,
                connection_options,
            }),
            Err(err) => return Err(err),
        }
    }

    pub fn get_databases(&mut self) -> Vec<Row> {
        self.client
            .query(
                "SELECT datname from pg_database WHERE datistemplate = false",
                &[],
            )
            .expect("Get databases")
    }

    pub fn get_tables_for_database(&mut self) -> Result<Vec<Row>, Error> {
        self.client.query(
            "SELECT tablename FROM pg_tables where schemaname = 'public'",
            &[],
        )
    }

    pub fn create_database_connection(
        &mut self,
        connection_options: PSQLConnectionOptions,
    ) -> Result<(), Error> {
        let client_result = Client::connect(
            format!(
                "host={} user={} dbname={}",
                connection_options.host, connection_options.user, connection_options.db_name
            )
            .as_str(),
            NoTls,
        );

        match client_result {
            Ok(client) => {
                self.client = client;
                self.connection_options = connection_options;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn get_table(&mut self, table_name: String) -> Result<Vec<Row>, Error> {
        self.client.query(
            "SELECT column_name FROM information_schema.columns where table_name = ($1)",
            &[&table_name],
        )
    }

    pub fn get_data(&mut self, table_name: &String) -> Result<Vec<Row>, Error> {
        self.client.query(
            format!("SELECT * FROM {table_name} LIMIT 10").as_str(),
            &[&table_name],
        )
    }
}
