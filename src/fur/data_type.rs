use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FurDataType {
    name: String,
    encoder: PathBuf,
    decoder: PathBuf,
}

impl FurDataType {
    pub fn new(name: String, encoder: &PathBuf, decoder: &PathBuf) -> FurDataType {
        FurDataType {
            name,
            encoder: encoder.to_path_buf(),
            decoder: decoder.to_path_buf(),
        }
    }

    pub fn clone(old: &FurDataType) -> FurDataType {
        FurDataType {
            name: old.name.clone(),
            encoder: old.encoder.clone(),
            decoder: old.decoder.clone(),
        }
    }
}
