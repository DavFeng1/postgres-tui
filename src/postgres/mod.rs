pub mod query;

use crate::app::PSQLConnectionOptions;
use postgres::{Client, Error, NoTls};

pub fn connect(connection_options: PSQLConnectionOptions) -> Result<Client, Error> {
    Client::connect(
        format!(
            "host={} user={}",
            connection_options.host, connection_options.user
        )
        .as_str(),
        NoTls,
    )
}
