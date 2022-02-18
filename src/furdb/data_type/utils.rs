use crate::furdb::FurDataType;
use bitvec::prelude::*;

impl FurDataType {
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
