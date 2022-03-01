use crate::furdb::FurTable;
use bitvec::prelude::*;
use std::{
    collections::HashMap,
    error::Error,
    io::{BufReader, Read, Seek, SeekFrom},
};

impl FurTable {
    pub fn add(&self, rows: &[HashMap<&str, &str>]) -> Result<(), Box<dyn Error>> {
        let table_info = self.get_info()?;

        let mut rows_bin = BitVec::<u8, Msb0>::new();

        for row in rows {
            let mut row_bin = self.add_row(row, table_info.get_columns())?;
            rows_bin.append(&mut row_bin);
        }

        let bytes: Vec<u8> = rows_bin.into();
        self.write_data(&bytes)?;

        Ok(())
    }

    pub fn get_row_bin(
        &self,
        index: u64,
    ) -> Result<HashMap<String, BitVec<u8, Msb0>>, Box<dyn Error>> {
        let mut result = HashMap::<String, BitVec<u8, Msb0>>::new();

        let row_size = self.get_row_size()? / 8;

        let data_file_path = Self::get_data_file_path(&self.dir);
        let mut data_file = BufReader::new(std::fs::File::open(&data_file_path)?);

        let table_info = self.get_info()?;

        let row_start = index * row_size as u64;

        data_file.seek(SeekFrom::Start(row_start))?;

        let mut buf = vec![0u8; row_size];

        data_file.read_exact(&mut buf)?;
        let row_bin: BitVec<u8, Msb0> = BitVec::from_slice(&buf);

        let mut column_start = 0;
        for column in table_info.get_columns() {
            let column_size = column.get_size() as usize;

            let data_bin = &row_bin[column_start..(column_start + column_size)];
            let data_bin = BitVec::from(data_bin);
            column_start += column_size;

            result.insert(column.get_id(), data_bin);
        }

        Ok(result)
    }

    pub fn get_rows_bin(
        &self,
        indices: Vec<u64>,
    ) -> Result<Vec<HashMap<String, BitVec<u8, Msb0>>>, Box<dyn Error>> {
        let mut results = Vec::<HashMap<String, BitVec<u8, Msb0>>>::new();

        for index in indices {
            let result = self.get_row_bin(index)?;

            results.push(result);
        }

        Ok(results)
    }

    pub fn get_bin(&self) -> Result<Vec<HashMap<String, BitVec<u8, Msb0>>>, Box<dyn Error>> {
        let row_size = self.get_row_size()? / 8;

        let data_file_path = Self::get_data_file_path(&self.dir);

        let metadata = std::fs::metadata(&data_file_path)?;
        let data_file_size = metadata.len();

        let indices: Vec<u64> = (0..data_file_size / row_size as u64).collect();

        let results = self.get_rows_bin(indices)?;

        Ok(results)
    }

    pub fn get(&self) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
        let rows_bin = self.get_bin()?;
        let mut results = Vec::<HashMap<String, String>>::new();

        let table_info = self.get_info()?;

        for row_bin in rows_bin {
            let mut row = HashMap::new();

            for column in table_info.get_columns() {
                let data_type = column.get_data_type();

                let data_bin = row_bin.get(&column.get_id()).unwrap();
                let data = data_type.decode(data_bin, table_info.get_converter_server())?;

                row.insert(column.get_id(), data);
            }

            results.push(row);
        }

        Ok(results)
    }

    pub fn delete_all_data(&self) -> Result<(), Box<dyn Error>> {
        let data_file_path = Self::get_data_file_path(&self.dir);
        std::fs::write(data_file_path, "")?;

        Ok(())
    }
}
