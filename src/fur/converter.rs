use bit_vec::BitVec;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Converter {
    encoder: PathBuf,
    decoder: PathBuf,
}

impl Converter {
    pub fn new(encoder: PathBuf, decoder: PathBuf) -> Converter {
        Converter { encoder, decoder }
    }

    pub fn encode(&self, data: String) -> BitVec {
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

        println!("{:?}", binary);

        binary
    }

    pub fn decode(&self, bits: BitVec) -> String {
        String::from("")
    }
}
