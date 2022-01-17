use std::env;

mod fur;
use fur::FurDB;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args);

    let db_path: String = "E:\\Home\\Repositories\\fur\\TestDB".to_string();

    let db: FurDB = FurDB::new(&db_path);

    println!("Name: {0}", FurDB::get_name(&db));
    println!("Description: {0}", FurDB::get_description(&db));

    println!("Tables: {:?}", FurDB::get_tables(&db));
}
