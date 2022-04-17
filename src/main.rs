extern crate furdb;

mod operations;
use std::error::Error;

use operations::*;

fn main() -> Result<(), Box<dyn Error>> {
    let db = create_db()?;
    check_db(&db)?;
    let mut tb = create_table(&db)?;
    check_table(&mut tb)?;
    delete_data(&tb)?;
    add_data(&mut tb)?;
    println!();
    get_data(&mut tb)?;

    check_sortfile(&mut tb)?;

    // _converter_test()?;

    Ok(())
}
