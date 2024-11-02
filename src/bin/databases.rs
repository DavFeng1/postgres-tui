use postgres::{Client, Error, NoTls};

// Using this as a script to test out postgres connection
// Run with cargo run --bin postgres
pub fn main() -> Result<(), Error> {
    println!("testing postgres");

    let user = "feng";
    let host = "localhost";

    let mut client = Client::connect(format!("host={} user={}", host, user).as_str(), NoTls)?;

    let result = client.query(
        "SELECT datname FROM pg_database WHERE datallowconn = true",
        &[],
    )?;

    for row in result {
        let name: String = row.get(0);
        println!("found database: {}", name);
    }

    Ok(())
}
