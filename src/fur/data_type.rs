use std::{path::PathBuf, process::Output};

use bit_vec::BitVec;
use serde::{Deserialize, Serialize};
use serde_closure::{traits::Fn, Fn};

#[derive(Serialize, Deserialize)]
#[serde(bound(
    serialize = "&'a (dyn Fn<String, Output = BitVec> + 'a): Serialize",
    deserialize = "&'a (dyn Fn<String, Output = BitVec> + 'a): Deserialize<'de>",
))]
pub struct FurDataType<'a> {
    name: String,
    encoder: &'a dyn Fn<String, Output = BitVec>,
    decoder: PathBuf,
}

impl FurDataType<'_> {
    pub fn new<'a>(
        name: String,
        encoder: &'a dyn Fn<String, Output = BitVec>,
        decoder: &'a PathBuf,
    ) -> FurDataType<'a> {
        FurDataType {
            name,
            encoder,
            decoder: decoder.to_path_buf(),
        }
    }

    pub fn clone<'a>(old: &'a FurDataType) -> FurDataType<'a> {
        FurDataType {
            name: old.name.clone(),
            encoder: old.encoder.clone(),
            decoder: old.decoder.clone(),
        }
    }

    pub fn get_encoder(&self) -> &dyn Fn<String, Output = BitVec> {
        self.encoder
    }
}
