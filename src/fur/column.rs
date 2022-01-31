use serde::{Deserialize, Serialize};

use super::data_type::FurDataType;

use bit_vec::BitVec;
use serde_closure::{traits::Fn, Fn};

#[derive(Serialize, Deserialize)]
#[serde(bound(
    serialize = "&'a (dyn Fn<String, Output = BitVec> + 'a): Serialize",
    deserialize = "&'a (dyn Fn<String, Output = BitVec> + 'a): Deserialize<'de>",
))]
pub struct FurColumn<'a> {
    name: String,
    description: String,
    size: u128,
    data_type: FurDataType<'a>,
}

impl FurColumn<'_> {
    pub fn new<'a>(
        name: String,
        description: Option<String>,
        size: u128,
        data_type: &'a FurDataType,
    ) -> FurColumn<'a> {
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

    pub fn get_data_type(&self) -> &FurDataType {
        &self.data_type
    }
}
