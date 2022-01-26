use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::table_info::FurTableInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct FurTable {
    dir: PathBuf,
}

impl FurTable {
    pub fn new<'a>(
        dir: &'a PathBuf,
        table_info: Option<FurTableInfo>,
    ) -> std::io::Result<FurTable> {
        if !dir.exists() {
            std::fs::create_dir(dir)?;
        }

        let table_info_file_path = Self::get_info_file_path(dir);

        if !table_info_file_path.exists() {
            let table_name = dir
                .file_name()
                .unwrap_or(std::ffi::OsStr::new(""))
                .to_str()
                .unwrap_or("")
                .to_string();

            let table_info_contents = serde_json::to_string(
                &table_info.unwrap_or(FurTableInfo::new(table_name, None, None)),
            )?;

            std::fs::write(table_info_file_path, table_info_contents)?;
        }

        let data_file_path = Self::get_data_file_path(dir);

        if !data_file_path.exists() {
            std::fs::write(data_file_path, "")?;
        }

        Ok(FurTable {
            dir: dir.to_path_buf(),
        })
    }

    pub fn get_info(&self) -> std::io::Result<FurTableInfo> {
        let table_info_file_path = Self::get_info_file_path(&self.dir);

        let table_info_contents_raw = std::fs::read_to_string(&table_info_file_path)?;

        let table_info_contents: serde_json::Value =
            serde_json::from_str(&table_info_contents_raw)?;

        let value = serde_json::from_value(table_info_contents)?;

        Ok(value)
    }
}

impl FurTable {
    fn get_info_file_path(dir: &PathBuf) -> PathBuf {
        let mut table_info_file_path = dir.clone();
        table_info_file_path.push("fur_table.json");

        table_info_file_path
    }

    fn get_data_file_path(dir: &PathBuf) -> PathBuf {
        let mut data_file_path = dir.clone();
        data_file_path.push("data.fur");

        data_file_path
    }
}
