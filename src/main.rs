#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use chip8_webgpu::{
    chip8::{
        decoder::bit_manipulation::group_in_raw_instructions,
        disassembler::disassemble::disassemble_raw_instructions,
    },
    utilities::storage,
};

fn main() {
    let rom_data = storage::read_rom("ibm_logo").expect("Unable to load rom");

    let instructions = group_in_raw_instructions(&rom_data);

    let disassembled_instructions = disassemble_raw_instructions(&instructions);

    for instruction in disassembled_instructions {
        println!("{:?}", instruction);
    }
}
