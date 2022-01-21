use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FurDBInfo {
    name: String,
    description: String,
}

impl FurDBInfo {
    pub fn new(name: String, description: String) -> FurDBInfo {
        FurDBInfo { name, description }
    }
}
