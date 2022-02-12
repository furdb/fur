use crate::furdb::FurTableInfo;
use std::path::PathBuf;

#[derive(Debug)]
pub struct FurTable {
    dir: PathBuf,
}

mod operations;
mod utils;

impl FurTable {
    pub fn new(dir: PathBuf, table_info: Option<FurTableInfo>) -> std::io::Result<FurTable> {
        Self::ensure_table_files(&dir, table_info)?;

        Ok(FurTable { dir })
    }

    pub fn get_info(&self) -> std::io::Result<FurTableInfo> {
        let table_info_file_path = Self::get_info_file_path(&self.dir);
        let table_info_contents_raw = std::fs::read_to_string(&table_info_file_path)?;
        let table_info_contents = serde_json::from_str(&table_info_contents_raw)?;
        let table_info = serde_json::from_value(table_info_contents)?;

        Ok(table_info)
    }
}
