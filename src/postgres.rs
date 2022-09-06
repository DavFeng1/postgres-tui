use postgres::{Client, NoTls};

pub fn connect() -> Option<Client> {
    let client = Client::connect("host=localhost user=postgres", NoTls);

    client.ok()
}

