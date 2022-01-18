use std::fs;
use std::path::PathBuf;

pub struct FurDB {
    dir: PathBuf,
}

impl FurDB {
    pub fn new(dir: PathBuf) -> Result<FurDB, &'static str> {
        // TODO: Handle the errors here
        if false {
            return Err("Error occoured");
        }

        Ok(FurDB { dir: dir })
    }

    pub fn get_name(db: &FurDB) -> String {
        Self::get_info(db, "name")
    }

    pub fn get_description(db: &FurDB) -> String {
        Self::get_info(db, "description")
    }

    pub fn get_tables(db: &FurDB) -> Vec<String> {
        let mut tables = Vec::new();

        for file in fs::read_dir(&db.dir).unwrap() {
            let file_name = file.unwrap();
            let metadata = fs::metadata(&file_name.path());

            if metadata.unwrap().is_dir() {
                tables.push(file_name.file_name().to_string_lossy().to_string());
            }
        }

        tables
    }

    fn get_info_file_path(db: &FurDB) -> PathBuf {
        let mut db_info_file_path = db.dir.clone();
        db_info_file_path.push("fur.json");

        db_info_file_path
    }

    fn get_info(db: &FurDB, property: &str) -> String {
        let db_info_file_path = Self::get_info_file_path(db);

        let db_info_contents_raw =
            fs::read_to_string(&db_info_file_path).unwrap_or_else(|err| err.to_string());

        let db_info_contents: serde_json::Value =
            serde_json::from_str(&db_info_contents_raw).expect("");

        let value = db_info_contents.get(property).unwrap().to_string();

        value
    }
}
