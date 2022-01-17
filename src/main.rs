use std::env;

mod fur_database;
use fur_database::FurDatabase;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let db_path: String = "E:\\Home\\Repositories\\fur\\TestDB".to_string();

    let db: FurDatabase = FurDatabase::new(db_path);

    println!("{0}", FurDatabase::get_name(&db));
    println!("{0}", FurDatabase::get_description(&db));
}
