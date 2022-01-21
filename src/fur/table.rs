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
            let table_info_contents = serde_json::to_string(
                &table_info.unwrap_or(FurTableInfo::new(String::from(""), String::from(""), None)),
            )?;

            std::fs::write(table_info_file_path, table_info_contents)?;
        }

        Ok(FurTable {
            dir: dir.to_path_buf(),
        })
    }
}

impl FurTable {
    fn get_info_file_path(dir: &PathBuf) -> PathBuf {
        let mut table_info_file_path = dir.clone();
        table_info_file_path.push("fur_table.json");

        table_info_file_path
    }
}
