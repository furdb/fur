use std::fs;

pub struct FurDatabase {
    directory: String,
}

impl FurDatabase {
    pub fn new(directory: String) -> FurDatabase {
        FurDatabase {
            directory: directory,
        }
    }

    pub fn get_name(database: &FurDatabase) -> String {
        Self::get_info(database, "name")
    }

    pub fn get_description(database: &FurDatabase) -> String {
        Self::get_info(database, "description")
    }

    fn get_info_file_path(database: &FurDatabase) -> String {
        database.directory.clone() + &"\\fur.json"
    }

    pub fn get_info(database: &FurDatabase, property: &str) -> String {
        let db_info_path = Self::get_info_file_path(database);
        let db_info_contents_raw =
            fs::read_to_string(&db_info_path).expect("Something went wrong while reading the file");

        let db_info_contents: serde_json::Value = serde_json::from_str(&db_info_contents_raw)
            .expect("Something went wrong while parsing JSON");

        let property = db_info_contents.get(property).unwrap().to_string();

        property
    }
}
