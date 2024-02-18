use crate::chip8::{
    basic_types::RawInstruction, decoder::decode::decode_instructions,
    instructions::base_instruction::DynamicInstruction,
};

pub fn disassemble_raw_instructions(instructions: &[RawInstruction]) -> Vec<String> {
    decode_instructions(instructions)
        .iter()
        .map(|instr| instruction_to_disassembled_str(&instr))
        .collect()
}

pub fn instruction_to_disassembled_str(instruction: &DynamicInstruction) -> String {
    format!(
        "{}  {}",
        instruction.mnemonic_name(),
        instruction.parameters_str()
    )
}
