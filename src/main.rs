use std::path::PathBuf;

mod fur;
use fur::FurDB;

fn main() -> std::io::Result<()> {
    // Set the path to the DB
    let db_path = PathBuf::from("E:\\Home\\Repositories\\fur\\TestDBs\\TestDB2");

    // Create the DB object
    let db = FurDB::new(db_path);

    // Create the DB structure
    // Note: The DB should NOT already exist. Overwrites the DB info if the info file exists
    db.create(
        String::from("Another DB of mine!"),
        String::from("Another sample description"),
    )?;

    // Get the DB info
    // Note: DB should already exist
    println!("Name: {0}", db.get_name().unwrap());
    println!("Description: {0}", db.get_description().unwrap());
    println!("Tables: {:?}", db.get_tables().unwrap());

    Ok(())
}
