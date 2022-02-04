use serde::{Deserialize, Serialize};

use super::column::FurColumn;

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    ) -> std::io::Result<FurTableInfo> {
        let mut row_size: u128 = 0;

        for column in columns.clone().unwrap() {
            row_size += column.get_size();
        }

        if row_size % 8 != 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Unsupported,
                "Sum of the sizes of all the columns should be a multiple of 8.",
            ));
        }

        Ok(FurTableInfo {
            name,
            description: description.unwrap_or(String::from("")),
            columns: columns.unwrap_or(Vec::new()),
        })
    }

    pub fn get_columns(&self) -> Vec<FurColumn> {
        self.columns.clone()
    }
}
