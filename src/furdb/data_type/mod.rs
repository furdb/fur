use std::error::Error;

use bitvec::prelude::*;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FurDataType {
    id: String,
    converter: String,
}

mod operations;
mod utils;

impl FurDataType {
    pub fn new(id: &str, converter: &str) -> FurDataType {
        let id = String::from(id);
        let converter = String::from(converter);

        FurDataType { id, converter }
    }
}
