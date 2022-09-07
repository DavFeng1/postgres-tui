use postgres::{Client, Error, NoTls};

pub fn connect(host: &str, user: &str) -> Result<Client, Error> {
    Client::connect(
        format!("host={} user={}", host, user).as_str(),
        NoTls
    )
}

