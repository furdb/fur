use super::Converter;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FurDataType {
    name: String,
    converter: Converter,
}

impl FurDataType {
    pub fn new(name: String, converter: Converter) -> FurDataType {
        FurDataType { name, converter }
    }

    pub fn get_converter(&self) -> Converter {
        self.converter.clone()
    }
}
