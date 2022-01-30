use serde::{Deserialize, Serialize};

use super::column::FurColumn;

#[derive(Debug, Serialize, Deserialize)]
pub struct FurTableInfo {
    name: String,
    description: String,
    columns: Vec<FurColumn>,
}

impl FurTableInfo {
    pub fn new(
        name: String,
        description: Option<String>,
        columns: Option<Vec<FurColumn>>,
    ) -> FurTableInfo {
        FurTableInfo {
            name,
            description: description.unwrap_or(String::from("")),
            columns: columns.unwrap_or(Vec::new()),
        }
    }

    pub fn get_columns(&self) -> &Vec<FurColumn> {
        &self.columns
    }
}
