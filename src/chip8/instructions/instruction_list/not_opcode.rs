use crate::chip8::{
    instructions::base_instruction::IsInstruction, machine_state::state::Chip8VMState,
};

#[derive(Debug)]
pub struct NotOpcode {}

impl IsInstruction for NotOpcode {
    fn mnemonic_name(&self) -> &'static str {
        "NOTOP"
    }

    fn parameters_str(&self) -> String {
        "".to_string()
    }

    fn execute(&self, _: &mut Chip8VMState) {
        panic!("Tried to execute invalid data - instruction is not a supported Opcode");
    }
}
