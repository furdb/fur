use std::path::PathBuf;

mod fur;
use crate::fur::{FurColumn, FurDB, FurDBInfo, FurDataType, FurTableInfo};

mod models;
use models::Person;

fn main() -> std::io::Result<()> {
    let db_path = PathBuf::from("E:\\Home\\Repositories\\fur\\TestDBs\\PersonData");
    let db = FurDB::new(
        &db_path,
        Some(&FurDBInfo::new(
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

    // We want some columns, based on the Person struct
    let integer_data_type = FurDataType::new(
        String::from("Integer"),
        &PathBuf::from(""),
        &PathBuf::from(""),
    );

    let person_id_column = FurColumn::new(
        String::from("ID"),
        Some(String::from("")),
        5,
        &integer_data_type,
    );

    let person_fav_num_column = FurColumn::new(
        String::from("Favourite Number"),
        Some(String::from("")),
        11,
        &integer_data_type,
    );

    let table_name = String::from("PersonInfo");
    let table_info = FurTableInfo::new(
        String::from("Person Info"),
        Some(String::from(
            "Information regarding some people and their favourite numbers!",
        )),
        Some(Vec::new()),
    );
    let tb = db.get_table(table_name, Some(table_info))?;

    let tb_info = tb.get_info()?;
    println!("{:?}", tb_info);

    let p1 = Person::new(7, 18);

    Ok(())
}
