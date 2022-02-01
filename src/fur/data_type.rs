use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FurDataType {
    name: String,
    encoder: PathBuf,
    decoder: PathBuf,
}

impl FurDataType {
    pub fn new(name: String, encoder: PathBuf, decoder: PathBuf) -> FurDataType {
        FurDataType {
            name,
            encoder,
            decoder,
        }
    }

    pub fn get_encoder(&self) -> PathBuf {
        self.encoder.clone()
    }
}
