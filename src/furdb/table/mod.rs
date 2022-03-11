use crate::furdb::FurTableInfo;
use std::{
    error::Error,
    fs::{File, OpenOptions},
    path::PathBuf,
};

#[derive(Debug)]
pub struct FurTable {
    dir: PathBuf,
    data_file: File,
    data_file_size: u64,
    table_info: FurTableInfo,
}

mod operations;
mod utils;

impl FurTable {
    pub fn new(dir: PathBuf, table_info: FurTableInfo) -> Result<FurTable, Box<dyn Error>> {
        Self::ensure_table_files(&dir, Some(&table_info))?;

        let data_file_path = Self::get_data_file_path(&dir);
        let data_file = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open(data_file_path)?;
        let data_file_size = Self::get_data_file_size(&dir)?;

        // let info_file_path = Self::get_info_file_path(&dir);
        // let info_file = OpenOptions::new()
        //     .read(true)
        //     .write(true)
        //     .append(true)
        //     .open(info_file_path)?;

        Ok(FurTable {
            dir,
            data_file,
            data_file_size,
            table_info,
        })
    }

    pub fn get_info(&mut self) -> std::io::Result<&FurTableInfo> {
        // let mut table_info_contents_raw = String::new();
        // self.info_file.seek(SeekFrom::Start(0))?;
        // self.info_file
        //     .read_to_string(&mut table_info_contents_raw)?;
        // let table_info_contents = serde_json::from_str(&table_info_contents_raw)?;
        // let table_info = serde_json::from_value(table_info_contents)?;

        Ok(&self.table_info)
    }
}
