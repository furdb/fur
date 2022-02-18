use crate::furdb::FurDataType;
use bitvec::prelude::*;
use std::error::Error;

impl FurDataType {
    pub fn encode(&self, data: &str, size: u128) -> Result<BitVec<u8, Msb0>, Box<dyn Error>> {
        let data: String = data.into();
        let url = format!("{}/encode?data={}&size={}", self.converter, data, size);

        let res = reqwest::blocking::get(url)?.text()?;

        let binary = Self::string_to_bitvec(&res);

        Ok(binary)
    }

    pub fn decode(&self, bits: &BitVec<u8, Msb0>) -> Result<String, Box<dyn Error>> {
        let bits = Self::bitvec_to_string(bits);

        let url = format!("{}/decode?binary={}", self.converter, bits);

        let res = reqwest::blocking::get(url)?.text()?;

        Ok(res)
    }
}
