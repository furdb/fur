use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FurDBInfo {
    name: String,
}

impl FurDBInfo {
    pub fn new(name: &str) -> FurDBInfo {
        let name = String::from(name);

        FurDBInfo { name }
    }
}
