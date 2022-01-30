use serde::{Deserialize, Serialize};

use super::data_type::FurDataType;

#[derive(Debug, Serialize, Deserialize)]
pub struct FurColumn {
    name: String,
    description: String,
    size: u128,
    data_type: FurDataType,
}

impl FurColumn {
    pub fn new(
        name: String,
        description: Option<String>,
        size: u128,
        data_type: &FurDataType,
    ) -> FurColumn {
        FurColumn {
            name,
            description: description.unwrap_or(String::from("")),
            size,
            data_type: FurDataType::clone(data_type),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}
