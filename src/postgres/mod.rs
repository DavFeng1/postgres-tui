pub mod query;

use postgres::{Client, Error, NoTls};
use crate::app::PSQLConnectionOptions;
use tokio_postgres::NoTls;

pub fn connect(connection_options: PSQLConnectionOptions) -> Result<Client, Error> {
    Client::connect(
        format!(
            "host={} user={}",
            connection_options.host,
            connection_options.user
        ).as_str(),
        NoTls
    )
}

