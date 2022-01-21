use serde::{Deserialize, Serialize};

use super::data_type::FurDataType;

#[derive(Debug, Serialize, Deserialize)]
pub struct FurColumn {
    name: String,
    description: String,
    size: u128,
    data_type: FurDataType,
}
