use std::path::PathBuf;
use std::process;

mod fur;
use fur::FurDB;

// mod cli;
// use clap::Parser;

fn main() {
    // let _args = cli::Args::parse();

    let db_path = PathBuf::from("E:\\Home\\Repositories\\fur\\TestDB");

    let db = FurDB::new(db_path).unwrap_or_else(|err| {
        println!("Error accessing database: {0}", err);
        process::exit(1);
    });

    println!("Name: {0}", FurDB::get_name(&db));
    println!("Description: {0}", FurDB::get_description(&db));
    println!("Tables: {:?}", FurDB::get_tables(&db));
}
