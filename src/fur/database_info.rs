use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FurDBInfo {
    name: String,
    description: String,
}

impl FurDBInfo {
    pub fn new(name: String, description: Option<String>) -> FurDBInfo {
        FurDBInfo {
            name,
            description: description.unwrap_or(String::from("")),
        }
    }
}
