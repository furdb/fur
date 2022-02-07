use std::{collections::HashMap, io::Result, path::PathBuf};
mod furdb;
use furdb::{Converter, FurColumn, FurDB, FurDBInfo, FurDataType, FurTable, FurTableInfo};

fn main() -> Result<()> {
    let db = create_db()?;

    check_db(&db)?;

    let tb = create_table(&db)?;

    check_table(&tb)?;

    create_data(&tb)?;

    // converter_test()?;

    Ok(())
}

fn _converter_test() -> Result<()> {
    let converter = Converter::new(PathBuf::new(), PathBuf::new())?;

    let encoded = converter.encode("9", 10)?;

    println!("Encoded: {}", encoded);

    Ok(())
}

fn create_db() -> Result<FurDB> {
    let db_path = PathBuf::from("D:\\Home\\Repositories\\fur\\TestDBs\\PersonData");
    let db_info = FurDBInfo::new("Person Data");

    let db = FurDB::new(db_path, Some(db_info))?;

    Ok(db)
}

fn create_table(db: &FurDB) -> Result<FurTable> {
    let columns = create_columns()?;

    let table_name = "PersonInfo";
    let table_info = FurTableInfo::new("Person Info", Some(columns))?;

    let tb = db.get_table(table_name, Some(table_info))?;

    Ok(tb)
}

fn create_columns() -> Result<Vec<FurColumn>> {
    let integer_data_type = create_data_type()?;

    let person_id_column = FurColumn::new("id", Some("ID"), 5, integer_data_type.clone());

    let person_fav_num_column = FurColumn::new(
        "favourite_number",
        Some("Favourite Number"),
        11,
        integer_data_type.clone(),
    );

    Ok(vec![person_id_column, person_fav_num_column])
}

fn create_data_type() -> Result<FurDataType> {
    let converter = create_converter()?;

    let integer_data_type = FurDataType::new("Integer", converter.clone());

    Ok(integer_data_type)
}

fn create_converter() -> Result<Converter> {
    let encoder = PathBuf::from("");
    let decoder = PathBuf::from("");

    Converter::new(encoder, decoder)
}

fn create_data(tb: &FurTable) -> Result<()> {
    let p1_info = HashMap::from([
        (String::from("id"), String::from("7")),
        (String::from("favourite_number"), String::from("18")),
    ]);

    tb.add(p1_info)?;

    Ok(())
}

fn check_db(db: &FurDB) -> Result<()> {
    let db_info = db.get_info()?;
    println!("Database Info: {:?}", db_info);

    let db_tables = db.get_all_tables()?;
    println!("Database Tables: {:?}", db_tables);

    Ok(())
}

fn check_table(tb: &FurTable) -> Result<()> {
    let tb_info = tb.get_info()?;
    println!("{:?}", tb_info);

    Ok(())
}
