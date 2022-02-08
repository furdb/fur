use super::{FurDBInfo, FurTable, FurTableInfo};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct FurDB {
    dir: PathBuf,
}

impl FurDB {
    pub fn new(dir: PathBuf, db_info: Option<FurDBInfo>) -> std::io::Result<FurDB> {
        if !dir.exists() {
            std::fs::create_dir(&dir)?;
        }

        let db_info_file_path = Self::get_info_file_path(&dir);

        if !db_info_file_path.exists() {
            let db_name = dir
                .file_name()
                .unwrap_or(std::ffi::OsStr::new(""))
                .to_str()
                .unwrap_or("");

            let db_info = db_info.unwrap_or(FurDBInfo::new(db_name));
            let db_info_contents_raw = serde_json::to_string(&db_info)?;

            std::fs::write(db_info_file_path, db_info_contents_raw)?;
        }

        Ok(FurDB { dir })
    }

    pub fn get_all_tables(&self) -> std::io::Result<Vec<String>> {
        let mut tables = Vec::new();

        for file in std::fs::read_dir(&self.dir)? {
            let file_name = file?;
            let metadata = std::fs::metadata(&file_name.path());

            if metadata?.is_dir() {
                tables.push(file_name.file_name().to_string_lossy().to_string());
            }
        }

        Ok(tables)
    }

    pub fn get_table(
        &self,
        table_id: &str,
        table_info: Option<FurTableInfo>,
    ) -> std::io::Result<FurTable> {
        let mut table_dir_path = self.dir.clone();
        table_dir_path.push(table_id);
        let tb = FurTable::new(table_dir_path, table_info)?;

        Ok(tb)
    }

    pub fn get_info(&self) -> std::io::Result<FurDBInfo> {
        let db_info_file_path = Self::get_info_file_path(&self.dir);
        let db_info_contents_raw = std::fs::read_to_string(&db_info_file_path)?;
        let db_info_contents = serde_json::from_str(&db_info_contents_raw)?;
        let db_info = serde_json::from_value(db_info_contents)?;

        Ok(db_info)
    }
}

impl FurDB {
    fn get_info_file_path(dir: &PathBuf) -> PathBuf {
        let mut db_info_file_path = dir.clone();
        db_info_file_path.push("fur.json");

        db_info_file_path
    }
}