use crate::furdb::{Converter, FurDataType};
use std::{error::Error, path::PathBuf};

pub struct StandardFurTypes {}

impl StandardFurTypes {
    pub fn unsigned_integer() -> Result<FurDataType, Box<dyn Error>> {
        let encoder = PathBuf::new();
        let decoder = PathBuf::new();
        let converter = Converter::new(encoder, decoder)?;
        let data_type = FurDataType::new("Unsigned Integer", converter);

        Ok(data_type)
    }
}
