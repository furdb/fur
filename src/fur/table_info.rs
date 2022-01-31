use serde::{Deserialize, Serialize};

use super::column::FurColumn;

use bit_vec::BitVec;
use serde_closure::{traits::Fn, Fn};

#[derive(Serialize, Deserialize)]
#[serde(bound(
    serialize = "&'a (dyn Fn<String, Output = BitVec> + 'a): Serialize",
    // serialize = "&dyn Fn<String, Output = BitVec>: Serialize",
    deserialize = "&'a (dyn Fn<String, Output = BitVec> + 'a): Deserialize<'de>",
))]
pub struct FurTableInfo<'a> {
    name: String,
    description: String,
    columns: Vec<FurColumn<'a>>,
}

impl FurTableInfo<'_> {
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
