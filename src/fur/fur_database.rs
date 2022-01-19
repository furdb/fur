use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub struct FurDB {
    dir: PathBuf,
}

impl FurDB {
    pub fn new(dir: PathBuf) -> FurDB {
        FurDB { dir: dir }
    }

    pub fn get_name(&self) -> Result<String, Box<dyn Error>> {
        self.get_info("name")
    }

    pub fn get_description(&self) -> Result<String, Box<dyn Error>> {
        self.get_info("description")
    }

    pub fn get_tables(&self) -> Vec<String> {
        let mut tables = Vec::new();

        for file in fs::read_dir(&self.dir).unwrap() {
            let file_name = file.unwrap();
            let metadata = fs::metadata(&file_name.path());

            if metadata.unwrap().is_dir() {
                tables.push(file_name.file_name().to_string_lossy().to_string());
            }
        }

        tables
    }

    fn get_info_file_path(&self) -> PathBuf {
        let mut db_info_file_path = self.dir.clone();
        db_info_file_path.push("fur.json");

        db_info_file_path
    }

    fn get_info(&self, property: &str) -> Result<String, Box<dyn Error>> {
        let db_info_file_path = self.get_info_file_path();

        let db_info_contents_raw = fs::read_to_string(&db_info_file_path)?;

        let db_info_contents: serde_json::Value = serde_json::from_str(&db_info_contents_raw)?;

        let value = db_info_contents.get(property).unwrap().to_string();

        Ok(value)
    }
}
