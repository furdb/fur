use std::error::Error;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FurDataType {
    id: String,
    converter_endpoint: String,
}

mod operations;
mod utils;

impl FurDataType {
    pub fn new(id: &str, converter_endpoint: &str) -> Result<FurDataType, Box<dyn Error>> {
        let id = String::from(id);
        let converter_endpoint = String::from(converter_endpoint);

        Ok(FurDataType {
            id,
            converter_endpoint,
        })
    }
}
