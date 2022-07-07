extern crate furdb;

mod operations;
use std::error::Error;

use operations::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db = create_db().await?;
    check_db(&db).await?;
    let mut tb = create_table(&db).await?;
    check_table(&mut tb).await?;
    delete_data(&tb).await?;
    add_data(&mut tb).await?;
    println!();
    get_data(&mut tb).await?;

    delete_sortfile(&mut tb)?;
    check_sortfile(&mut tb).await?;

    let column = tb.get_info()?.get_columns()[0].clone();
    check_query(&mut tb, &column).await?;

    Ok(())
}
