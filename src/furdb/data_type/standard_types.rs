use crate::furdb::{Converter, FurDataType};
use std::path::PathBuf;

pub struct StandardFurTypes {}

impl StandardFurTypes {
    pub fn unsigned_integer() -> std::io::Result<FurDataType> {
        let encoder = PathBuf::new();
        let decoder = PathBuf::new();
        let converter = Converter::new(encoder, decoder)?;
        let data_type = FurDataType::new("Unsigned Integer", converter);

        Ok(data_type)
    }
}
