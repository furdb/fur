use super::converter::Converter;
use super::table_info::FurTableInfo;
use bitvec::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::OpenOptions, io::Write, path::PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FurTable {
    dir: PathBuf,
}

impl FurTable {
    pub fn new(dir: PathBuf, table_info: Option<FurTableInfo>) -> std::io::Result<FurTable> {
        if !dir.exists() {
            std::fs::create_dir(&dir)?;
        }

        let table_info_file_path = Self::get_info_file_path(&dir);

        if !table_info_file_path.exists() {
            let table_name = dir
                .file_name()
                .unwrap_or(std::ffi::OsStr::new(""))
                .to_str()
                .unwrap_or("")
                .to_string();

            let table_info = &table_info.unwrap_or(FurTableInfo::new(&table_name, None, None)?);

            let table_info_contents = serde_json::to_string(table_info)?;

            std::fs::write(table_info_file_path, table_info_contents)?;
        }

        let data_file_path = Self::get_data_file_path(&dir);

        if !data_file_path.exists() {
            std::fs::write(data_file_path, "")?;
        }

        Ok(FurTable { dir })
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
    pub fn add(&self, data: HashMap<String, String>) -> std::io::Result<()> {
        let table_info = self.get_info()?;

        let mut raw_binary: BitVec<u8, Msb0> = BitVec::new();

        for column in table_info.get_columns() {
            let column_id = column.get_id();
            let default_value = String::from("");
            let value = data.get(&column_id).unwrap_or(&default_value);

            let converter = column.get_data_type().get_converter();

            let column_binary = converter.encode(value.clone(), column.get_size())?;

            raw_binary.append(&mut column_binary.clone());
        }

        // raw_binary should be a multiple of 8

        let bytes: Vec<u8> = raw_binary.into();

        println!("Raw binary: {:?}", bytes);

        let data_file_path = Self::get_data_file_path(&self.dir);

        let mut data_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(data_file_path)?;

        data_file.write(&bytes)?;

        Ok(())
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
