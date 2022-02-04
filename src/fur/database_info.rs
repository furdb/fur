use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FurDBInfo {
    name: String,
    description: String,
}

impl FurDBInfo {
    pub fn new(name: &str, description: Option<&str>) -> FurDBInfo {
        FurDBInfo {
            name: String::from(name),
            description: String::from(description.unwrap_or("")),
        }
    }
}
