use std::path::PathBuf;

mod fur;
use fur::FurDB;

mod models;
use models::Person;

use crate::fur::{FurDBInfo, FurTableInfo};

fn main() -> std::io::Result<()> {
    let db_path = PathBuf::from("E:\\Home\\Repositories\\fur\\TestDBs\\PersonData");
    let db = FurDB::new(
        &db_path,
        Some(FurDBInfo::new(
            String::from("Person Data"),
            Some(String::from(
                "Database for storing the data regarding various people.",
            )),
        )),
    )?;

    let db1_info = db.get_info()?;

    println!("Database Info: {:?}", db1_info);

    let db_tables = db.get_all_tables()?;

    println!("Database Tables: {:?}", db_tables);

    let tb_name = String::from("PersonInfo");
    let tb = db.get_table(
        tb_name,
        Some(FurTableInfo::new(
            String::from("Person Info"),
            Some(String::from(
                "Information regarding some people and their favourite numbers!",
            )),
            Some(Vec::new()),
        )),
    )?;

    let tb_info = tb.get_info()?;

    println!("{:?}", tb_info);

    Ok(())
}
