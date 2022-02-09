use crate::furdb::{FurColumn, FurTable, FurTableInfo};
use bitvec::prelude::*;
use std::{collections::HashMap, fs::OpenOptions, io::Write, path::PathBuf};

impl FurTable {
    pub(super) fn ensure_table_files(
        dir: &PathBuf,
        table_info: Option<FurTableInfo>,
    ) -> std::io::Result<()> {
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

            let table_info = &table_info.unwrap_or(FurTableInfo::new(&table_name, None)?);
            let table_info_contents = serde_json::to_string(table_info)?;

            std::fs::write(table_info_file_path, table_info_contents)?;
        }

        let data_file_path = Self::get_data_file_path(&dir);
        if !data_file_path.exists() {
            std::fs::write(data_file_path, "")?;
        }

        Ok(())
    }

    pub(super) fn add_row(
        &self,
        data: &HashMap<&str, &str>,
        columns: &[FurColumn],
    ) -> std::io::Result<BitVec<u8, Msb0>> {
        let mut row_binary_raw = BitVec::new();

        for column in columns {
            let column_id = column.get_id();
            let column_id = column_id.as_str();

            let data = data.get(column_id).unwrap_or(&&"");

            let data_type = column.get_data_type();
            let converter = data_type.get_converter();

            let mut column_binary = converter.encode(data, column.get_size())?;
            row_binary_raw.append(&mut column_binary);
        }

        Ok(row_binary_raw)
    }

    pub(super) fn get_row_size(&self) -> std::io::Result<usize> {
        let table_info = self.get_info()?;
        let mut size = 0;

        for column in table_info.get_columns() {
            size += column.get_size();
        }

        Ok(size as usize)
    }

    pub(super) fn write_data(&self, bytes: &Vec<u8>) -> std::io::Result<()> {
        let data_file_path = Self::get_data_file_path(&self.dir);

        let mut data_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(data_file_path)?;

        data_file.write(&bytes)?;

        Ok(())
    }

    pub(super) fn get_info_file_path(dir: &PathBuf) -> PathBuf {
        let mut table_info_file_path = dir.clone();
        table_info_file_path.push("fur_table.json");

        table_info_file_path
    }

    pub(super) fn get_data_file_path(dir: &PathBuf) -> PathBuf {
        let mut data_file_path = dir.clone();
        data_file_path.push("data.fur");

        data_file_path
    }
}
