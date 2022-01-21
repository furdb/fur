use serde::{Deserialize, Serialize};

use super::column::FurColumn;

#[derive(Debug, Serialize, Deserialize)]
pub struct FurTableInfo {
    name: String,
    description: String,
    columns: Vec<FurColumn>,
}

impl FurTableInfo {
    pub fn new(name: String, description: String, columns: Option<Vec<FurColumn>>) -> FurTableInfo {
        FurTableInfo {
            name,
            description,
            columns: columns.unwrap_or(Vec::new()),
        }
    }
}
