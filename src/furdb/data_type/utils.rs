use crate::furdb::FurDataType;
use bitvec::prelude::*;
use std::error::Error;

impl FurDataType {
    pub(super) fn resize(
        bits: BitVec<u8, Msb0>,
        size: u128,
    ) -> Result<BitVec<u8, Msb0>, Box<dyn Error>> {
        let mut resized_bits = BitVec::new();

        for _ in 0..(size - bits.len() as u128) {
            resized_bits.push(false);
        }

        resized_bits.append(&mut bits.clone());

        Ok(resized_bits)
    }

    pub(super) fn string_to_bitvec(bits: &String) -> BitVec<u8, Msb0> {
        let mut binary = BitVec::<u8, Msb0>::new();

        for bit in bits.chars() {
            binary.push(bit == '1');
        }

        binary
    }

    pub(super) fn bitvec_to_string(bits: &BitVec<u8, Msb0>) -> String {
        let mut binary = String::new();

        for bit in bits {
            if *bit {
                binary.push('1');
            } else {
                binary.push('0');
            }
        }

        binary
    }
}
