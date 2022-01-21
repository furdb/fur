use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FurDataType {
    name: String,
    encoder: PathBuf,
    decoder: PathBuf,
}
