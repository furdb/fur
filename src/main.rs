use std::fs;

static DB_INFO_FILE_NAME: &str = "fur.json";

fn main() {
    let db_path: &str = "E:\\Home\\Repositories\\fur\\TestDB";

    let db_info = db_path.to_string() + &"\\".to_string() + DB_INFO_FILE_NAME;

    let contents = fs::read_to_string(db_info).expect("Something went wrong reading the file");

    let json: serde_json::Value =
        serde_json::from_str(&contents).expect("JSON was not well-formatted");

    println!("{0}", json);
}
