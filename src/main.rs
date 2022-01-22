use std::path::PathBuf;

mod fur;
use fur::FurDB;

fn main() -> std::io::Result<()> {
    let db_path1 = PathBuf::from("E:\\Home\\Repositories\\fur\\TestDBs\\TestDB");
    let db1 = FurDB::new(&db_path1, None)?;
    let db1_info = db1.get_info()?;

    println!("DB1: {:?}", db1_info);

    let db1_tables = db1.get_all_tables()?;

    println!("DB1 Tables: {:?}", db1_tables);

    let tb1_name = String::from("test_table");
    let tb1 = db1.get_table(tb1_name)?;
    let tb1_info = tb1.get_info()?;

    println!("{:?}", tb1_info);

    Ok(())
}
