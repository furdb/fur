use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct FurTable {
    dir: PathBuf,
}

mod operations;
mod utils;
