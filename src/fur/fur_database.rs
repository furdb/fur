use std::fs;

pub struct FurDB {
    directory: String,
}

impl FurDB {
    pub fn new(directory: &str) -> FurDB {
        FurDB {
            directory: directory.to_string(),
        }
    }

    pub fn get_name(database: &FurDB) -> String {
        Self::get_info(database, "name")
    }

    pub fn get_description(database: &FurDB) -> String {
        Self::get_info(database, "description")
    }

    pub fn get_tables(database: &FurDB) -> Vec<String> {
        let mut tables = Vec::new();

        for file in fs::read_dir(&database.directory).unwrap() {
            let file_name = file.unwrap();
            let metadata = fs::metadata(&file_name.path());

            if metadata.unwrap().is_dir() {
                tables.push(file_name.file_name().to_string_lossy().to_string());
            }
        }

        tables
    }

    fn get_info_file_path(database: &FurDB) -> String {
        database.directory.clone() + &"\\fur.json"
    }

    fn get_info(database: &FurDB, property: &str) -> String {
        let db_info_path = Self::get_info_file_path(database);
        let db_info_contents_raw =
            fs::read_to_string(&db_info_path).expect("Something went wrong while reading the file");

        let db_info_contents: serde_json::Value = serde_json::from_str(&db_info_contents_raw)
            .expect("Something went wrong while parsing JSON");

        let property = db_info_contents.get(property).unwrap().to_string();

        property
    }
}
