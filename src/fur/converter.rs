use bitvec::prelude::*;
use serde::{Deserialize, Serialize};
use std::{io::Result, path::PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Converter {
    encoder: PathBuf,
    decoder: PathBuf,
}

impl Converter {
    pub fn new(encoder: PathBuf, decoder: PathBuf) -> Converter {
        Converter { encoder, decoder }
    }

    pub fn encode(&self, data: String, size: u128) -> Result<BitVec> {
        // -- Placeholder function for integers. Need to use stdio for other data types --

        let mut my_int = data.parse::<i32>().unwrap();
        let mut reversed_binary: Vec<u8> = Vec::new();

        while my_int > 0 {
            reversed_binary.push((my_int % 2).try_into().unwrap());
            my_int >>= 1;
        }

        let mut binary = BitVec::new();

        reversed_binary.reverse();

        for bit in reversed_binary {
            binary.push(bit == 1);
        }

        let resized_binary = Self::resize(binary, size)?;

        Ok(resized_binary)
    }

    fn resize(bits: BitVec, size: u128) -> Result<BitVec> {
        let mut resized_bits = BitVec::new();

        for _ in 0..(size - (bits.len() as u128)) {
            resized_bits.push(false);
        }

        resized_bits.append(&mut bits.clone());

        Ok(resized_bits)
    }

    pub fn decode(&self, bits: BitVec) -> String {
        String::from("")
    }
}
