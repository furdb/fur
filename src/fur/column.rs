use serde::{Deserialize, Serialize};

use super::data_type::FurDataType;

#[derive(Clone, Debug, Serialize, Deserialize)]
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
        data_type: FurDataType,
    ) -> FurColumn {
        FurColumn {
            name,
            description: description.unwrap_or(String::from("")),
            size,
            data_type: data_type.clone(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_size(&self) -> u128 {
        self.size
    }

    pub fn get_data_type(&self) -> FurDataType {
        self.data_type.clone()
    }
}
