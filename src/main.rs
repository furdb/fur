use std::path::PathBuf;

mod fur;
use fur::{FurDB, FurDBInfo};

fn main() -> std::io::Result<()> {
    let db_path1 = PathBuf::from("E:\\Home\\Repositories\\fur\\TestDBs\\TestDB");
    let db_path2 = PathBuf::from("E:\\Home\\Repositories\\fur\\TestDBs\\TestDB2");

    let db1 = FurDB::new(&db_path1, None)?;

    let db2_info = FurDBInfo::new(String::from("Name"), String::from("Description"));
    let db2 = FurDB::new(&db_path2, Some(db2_info))?;

    let db1_info = db1.get_info()?;
    let db1_tables = db1.get_tables()?;

    let db2_info = db2.get_info()?;
    let db2_tables = db2.get_tables()?;

    println!("DB1: {:?}", db1_info);
    println!("DB1 Tables: {:?}", db1_tables);

    println!("DB1: {:?}", db2_info);
    println!("DB2 Tables: {:?}", db2_tables);

    Ok(())
}
