use std::error::Error;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FurDataType {
    id: String,
    converter: String,
}

mod operations;
mod utils;

impl FurDataType {
    pub fn new(id: &str, converter: &str) -> Result<FurDataType, Box<dyn Error>> {
        let id = String::from(id);
        let converter = String::from(converter);

        Ok(FurDataType { id, converter })
    }
}
