use crate::chip8::{
    basic_types::{Chip8Address, RawInstruction},
    decoder::decode::decode_instructions,
    instructions::base_instruction::DynamicInstruction,
};

pub fn disassemble_raw_instructions(instructions: &[RawInstruction]) -> Vec<String> {
    decode_instructions(instructions)
        .iter()
        .enumerate()
        .map(|(index, instr)| instruction_to_disassembled_str(instr, 512 + index as u16 * 2))
        .collect()
}

pub fn instruction_to_disassembled_str(
    instruction: &DynamicInstruction,
    instruction_address: Chip8Address,
) -> String {
    format!(
        "0x{:x}: {}  {}",
        instruction_address,
        instruction.mnemonic_name(),
        instruction.parameters_str()
    )
}
