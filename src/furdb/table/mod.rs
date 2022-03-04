use crate::furdb::FurTableInfo;
use std::{error::Error, fs::File, io::BufReader, path::PathBuf};

#[derive(Debug)]
pub struct FurTable {
    dir: PathBuf,
    data_file: BufReader<File>,
}

mod operations;
mod utils;

impl FurTable {
    pub fn new(dir: PathBuf, table_info: Option<FurTableInfo>) -> Result<FurTable, Box<dyn Error>> {
        Self::ensure_table_files(&dir, table_info)?;

        let data_file_path = Self::get_data_file_path(&dir);
        let data_file = BufReader::new(std::fs::File::open(&data_file_path)?);

        Ok(FurTable { dir, data_file })
    }

    pub fn get_info(&self) -> std::io::Result<FurTableInfo> {
        let table_info_file_path = Self::get_info_file_path(&self.dir);
        let table_info_contents_raw = std::fs::read_to_string(&table_info_file_path)?;
        let table_info_contents = serde_json::from_str(&table_info_contents_raw)?;
        let table_info = serde_json::from_value(table_info_contents)?;

        Ok(table_info)
    }
}
