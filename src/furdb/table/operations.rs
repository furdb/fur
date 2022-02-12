use crate::furdb::FurTable;
use bitvec::prelude::*;
use std::{
    collections::HashMap,
    io::{BufReader, Read, Seek, SeekFrom},
};

impl FurTable {
    pub fn add(&self, datas: &[HashMap<&str, &str>]) -> std::io::Result<()> {
        let table_info = self.get_info()?;

        let mut data_binary_raw: BitVec<u8, Msb0> = BitVec::new();

        for data in datas {
            let mut row_binary_raw = self.add_row(data, table_info.get_columns())?;
            data_binary_raw.append(&mut row_binary_raw);
        }

        let bytes: Vec<u8> = data_binary_raw.into();
        self.write_data(&bytes)?;

        Ok(())
    }

    pub fn get(&self) -> std::io::Result<Vec<HashMap<String, String>>> {
        let mut result = Vec::new();

        let row_size = self.get_row_size()? / 8;
        let data_file_path = Self::get_data_file_path(&self.dir);
        let mut data_file = BufReader::new(std::fs::File::open(&data_file_path)?);
        let metadata = std::fs::metadata(&data_file_path)?;
        let data_file_size = metadata.len();

        let table_info = self.get_info()?;

        for row_start in (0..data_file_size).step_by(row_size) {
            data_file.seek(SeekFrom::Start(row_start))?;
            let mut data = HashMap::<String, String>::new();

            let mut buf = vec![0u8, row_size as u8];
            data_file.read_exact(&mut buf)?;
            let row: BitVec<u8, Msb0> = BitVec::from_slice(&buf);

            let mut column_start = 0;
            for column in table_info.get_columns() {
                let column_size = column.get_size() as usize;
                let section = &row[column_start..(column_start + column_size)];
                let section = BitVec::from(section);
                column_start += column_size;

                let data_type = column.get_data_type();
                let converter = data_type.get_converter();

                let value = converter.decode(&section)?;

                data.insert(column.get_id(), value);
            }

            result.push(data);
        }

        Ok(result)
    }

    pub fn delete_all_data(&self) -> std::io::Result<()> {
        let data_file_path = Self::get_data_file_path(&self.dir);
        std::fs::write(data_file_path, "")?;

        Ok(())
    }
}
