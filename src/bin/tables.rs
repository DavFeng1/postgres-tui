use postgres::{Client, Error, NoTls};


// Using this as a script to test out postgres connection
// Run with cargo run --bin postgres
pub fn main() -> Result<(), Error> {
    println!("testing postgres");

    let user = "postgres";
    let host = "localhost";
    let dbname = "shakepay_backend";


    let mut client = Client::connect(format!("host={} user={} dbname={}", host, user, dbname).as_str(), NoTls)?;

    let result = client.query("SELECT tablename FROM pg_tables where schemaname = 'public'", &[])?;

    for row in result {
        let name: String = row.get(0);
        println!("found tables: {}", name);
    }

    Ok(())
}
