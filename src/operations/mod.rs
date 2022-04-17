use bitvec::prelude::*;
use furdb::{FurColumn, FurDB, FurDBInfo, FurDataType, FurTable, FurTableInfo};
use std::{collections::HashMap, error::Error, path::PathBuf};

pub fn _converter_test() -> Result<(), Box<dyn Error>> {
    let id = "long_string";
    let data_type = FurDataType::new(&id, Some(format!("http://localhost:5000/{}", id).as_str()))?;

    let data = "Hello World";
    let size = 104;

    println!("Data: {} | Size: {}", data, size);

    let encoded: BitVec<u8, Msb0> = data_type.encode(data, size, None)?;

    println!("Encoded: {}", encoded);

    let decoded = data_type.decode(&encoded, None)?;

    println!("Decoded: {}", decoded);

    Ok(())
}

pub fn create_db() -> Result<FurDB, Box<dyn Error>> {
    println!("Creating DB...");

    let db_path = PathBuf::from("D:\\Home\\Repositories\\FurDB\\TestDBs\\PersonData");
    let db_info = FurDBInfo::new("Person Data")?;

    let db = FurDB::new(db_path, Some(db_info))?;

    Ok(db)
}

pub fn create_table(db: &FurDB) -> Result<FurTable, Box<dyn Error>> {
    println!("Creating table...");

    let columns = create_columns()?;

    let table_id = "PersonInfo";
    let table_info =
        FurTableInfo::new("Person Info", Some("http://localhost:5000"), Some(columns))?;

    let tb = db.get_table(table_id, Some(table_info))?;

    tb.save_info()?;

    Ok(tb)
}

pub fn delete_data(tb: &FurTable) -> Result<(), Box<dyn Error>> {
    println!("Deleting data...");

    tb.delete_all_rows()?;
    Ok(())
}

pub fn create_columns() -> Result<Vec<FurColumn>, Box<dyn Error>> {
    println!("Creating columns...");

    let (long_string_data_type, integer_data_type) = create_data_types()?;

    let person_id_column = FurColumn::new("id", Some("ID"), 5, integer_data_type.clone())?;

    let person_fav_num_column = FurColumn::new(
        "favourite_number",
        Some("Favourite Number"),
        11,
        integer_data_type.clone(),
    )?;

    let person_name_column =
        FurColumn::new("name", Some("Name"), 40, long_string_data_type.clone())?;

    Ok(vec![
        person_id_column,
        person_name_column,
        person_fav_num_column,
    ])
}

pub fn create_data_types() -> Result<(FurDataType, FurDataType), Box<dyn Error>> {
    println!("Creating data types...");

    let long_string_data_type = FurDataType::new("long_string", None)?;

    let unsigned_integer_data_type = FurDataType::new("unsigned_integer", None)?;

    Ok((long_string_data_type, unsigned_integer_data_type))
}

pub fn add_data(tb: &mut FurTable) -> Result<(), Box<dyn Error>> {
    println!("Adding data...");

    let p_info = [
        HashMap::from([("id", "7"), ("favourite_number", "18"), ("name", "Bob")]),
        HashMap::from([("id", "6"), ("favourite_number", "11"), ("name", "John")]),
    ];

    tb.add(&p_info)?;

    Ok(())
}

pub fn get_data(tb: &mut FurTable) -> Result<(), Box<dyn Error>> {
    println!("Getting data...");

    let result = tb.get_all()?;

    for row in result {
        for column in tb.get_info()?.get_columns() {
            println!(
                "{}: {}",
                column.get_id(),
                row.get(&column.get_id()).unwrap()
            );
        }

        println!();
    }

    Ok(())
}

pub fn check_db(db: &FurDB) -> Result<(), Box<dyn Error>> {
    println!("Checking DB...");

    let db_info = db.get_info()?;
    println!("Database Info: {:?}", db_info);

    let db_tables = db.get_all_table_ids()?;
    println!("Database Tables: {:?}", db_tables);

    Ok(())
}

pub fn check_table(tb: &mut FurTable) -> Result<(), Box<dyn Error>> {
    println!("Checking table...");

    let tb_info = tb.get_info()?;
    println!("{:?}", tb_info);

    Ok(())
}

pub fn check_sortfile(tb: &mut FurTable) -> Result<(), Box<dyn Error>> {
    tb.generate_all_sortfile_contents()
}
