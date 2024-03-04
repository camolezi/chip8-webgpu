use crate::chip8::basic_types::{Byte, Chip8Address, ExpandedByte, RawInstruction};

pub fn get_byte_nibbles(byte: Byte) -> (Byte, Byte) {
    let first_nibble = byte >> 4;
    let second_nibble = byte & 0x0F;

    (first_nibble, second_nibble)
}

pub fn get_byte_bits(byte: Byte) -> ExpandedByte {
    let mut bits = [0; 8];

    for (i, bit) in bits.iter_mut().enumerate() {
        *bit = (byte >> (7 - i)) & 1;
    }

    bits
}

pub fn group_in_raw_instructions(bytes: &[Byte]) -> Vec<RawInstruction> {
    bytes
        .chunks_exact(2)
        .map(|b| -> RawInstruction { RawInstruction(b[0], b[1]) })
        .collect()
}

pub fn combine_nibbles((first_nibble, second_nibble): (Byte, Byte)) -> Byte {
    let shifted_in_place_first_nibble = first_nibble << 4;

    shifted_in_place_first_nibble | second_nibble
}

pub fn combine_address_nibbles(
    (first_nibble, second_nibble, third_nibble): (Byte, Byte, Byte),
) -> Chip8Address {
    let result_address: Chip8Address =
        ((first_nibble as u16) << 8) | ((second_nibble as u16) << 4) | (third_nibble as u16);

    result_address
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_byte_bits() {
        let input_byte = 56;
        let expected_result: ExpandedByte = [0, 0, 1, 1, 1, 0, 0, 0];
        assert_eq!(get_byte_bits(input_byte), expected_result);
    }
}
