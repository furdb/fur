use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

pub struct FurDB {
    dir: PathBuf,
}

#[derive(Serialize, Deserialize)]
struct FurDBInfo {
    name: String,
    description: String,
}

impl FurDBInfo {
    fn new(name: String, description: String) -> FurDBInfo {
        FurDBInfo { name, description }
    }
}

impl FurDB {
    pub fn new(dir: PathBuf) -> FurDB {
        FurDB { dir }
    }

    pub fn create(&self, name: String, description: String) -> std::io::Result<()> {
        if !self.dir.exists() {
            std::fs::create_dir(&self.dir)?;
        }

        let db_info_file_path = self.get_info_file_path();

        let db_info = FurDBInfo::new(name, description);

        let db_info_contents = serde_json::to_string(&db_info)?;

        fs::write(db_info_file_path, db_info_contents)?;

        Ok(())
    }

    pub fn get_name(&self) -> std::io::Result<String> {
        self.get_info("name")
    }

    pub fn get_description(&self) -> std::io::Result<String> {
        self.get_info("description")
    }

    pub fn get_tables(&self) -> std::io::Result<Vec<String>> {
        let mut tables = Vec::new();

        for file in fs::read_dir(&self.dir)? {
            let file_name = file?;
            let metadata = fs::metadata(&file_name.path());

            if metadata?.is_dir() {
                tables.push(file_name.file_name().to_string_lossy().to_string());
            }
        }

        Ok(tables)
    }

    fn get_info_file_path(&self) -> PathBuf {
        let mut db_info_file_path = self.dir.clone();
        db_info_file_path.push("fur.json");

        db_info_file_path
    }

    fn get_info(&self, property: &str) -> std::io::Result<String> {
        let db_info_file_path = self.get_info_file_path();

        let db_info_contents_raw = fs::read_to_string(&db_info_file_path)?;

        let db_info_contents: serde_json::Value = serde_json::from_str(&db_info_contents_raw)?;

        let value = String::from(db_info_contents.get(property).unwrap().as_str().unwrap());

        Ok(value)
    }
}
