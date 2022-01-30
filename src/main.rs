use std::{collections::HashMap, path::PathBuf};

mod fur;
use crate::fur::{FurColumn, FurDB, FurDBInfo, FurDataType, FurTableInfo};

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

    let integer_data_type = FurDataType::new(
        String::from("Integer"),
        &PathBuf::from(""),
        &PathBuf::from(""),
    );

    let person_id_column = FurColumn::new(
        String::from("id"),
        Some(String::from("ID")),
        5,
        &integer_data_type,
    );

    let person_fav_num_column = FurColumn::new(
        String::from("favourite_number"),
        Some(String::from("Favourite Number")),
        11,
        &integer_data_type,
    );

    let table_name = String::from("PersonInfo");
    let table_info = FurTableInfo::new(
        String::from("Person Info"),
        Some(String::from(
            "Information regarding some people and their favourite numbers!",
        )),
        Some(vec![person_id_column, person_fav_num_column]),
    );
    let tb = db.get_table(table_name, Some(table_info))?;

    let tb_info = tb.get_info()?;
    println!("{:?}", tb_info);

    let p1 = HashMap::from([("id", "7"), ("favourite_number", "18")]);

    tb.add(p1)?;

    Ok(())
}
