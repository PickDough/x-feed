use postgres::{Client, Error, NoTls};

fn main() -> Result<(), Error> {
    let mut client = Client::connect(
        "postgres://root@roach-1:26257/twitter?sslmode=disable",
        NoTls,
    )
    .unwrap();
    println!("Creating accounts table if it doesn't already exist.");
    // Create the "accounts" table.
    client.execute(
        "CREATE TABLE IF NOT EXISTS accounts (id UUID PRIMARY KEY, balance INT)",
        &[],
    )?;

    Ok(())
}
