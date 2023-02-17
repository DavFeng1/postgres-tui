pub mod query;

use crate::app::PSQLConnectionOptions;
use postgres::{Client, Error, NoTls};

pub fn connect(connection_options: PSQLConnectionOptions) -> Result<Client, Error> {
    if let Some(dbname) = connection_options.dbname {
        Client::connect(
            format!(
                "host={} user={} dbname={}",
                connection_options.host, connection_options.user, dbname
            )
            .as_str(),
            NoTls,
        )
    } else {
        Client::connect(
            format!(
                "host={} user={}",
                connection_options.host, connection_options.user,
            )
            .as_str(),
            NoTls,
        )
    }
}
