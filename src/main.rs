use std::path::PathBuf;

mod fur;
use fur::FurDB;

// mod cli;
// use clap::Parser;

fn main() {
    // let _args = cli::Args::parse();

    // Path to the DB
    let db_path = PathBuf::from("E:\\Home\\Repositories\\fur\\TestDB");

    // DB object
    let db = FurDB::new(db_path);

    // Get DB info
    println!("Name: {0}", db.get_name().unwrap());
    println!("Description: {0}", db.get_description().unwrap());
    println!("Tables: {:?}", db.get_tables());
}
