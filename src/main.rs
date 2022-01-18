use std::env;
use std::path::PathBuf;
use std::process;

mod fur;
use fur::FurDB;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args);

    let db_path = PathBuf::from("E:\\Home\\Repositories\\fur\\TestDB");

    let db = FurDB::new(db_path).unwrap_or_else(|err| {
        println!("Error accessing database: {0}", err);
        process::exit(1);
    });

    println!("Name: {0}", FurDB::get_name(&db));
    println!("Description: {0}", FurDB::get_description(&db));
    println!("Tables: {:?}", FurDB::get_tables(&db));
}
