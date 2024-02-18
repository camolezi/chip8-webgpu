use crate::chip8::instructions::base_instruction::IsInstruction;

#[derive(Debug)]
pub struct NotOpcode {}

impl IsInstruction for NotOpcode {
    fn full_name(&self) -> &'static str {
        "Not Opcode"
    }

    fn mnemonic_name(&self) -> &'static str {
        "NOTOP"
    }

    fn parameters_str(&self) -> String {
        "".to_string()
    }

    fn execute(&self) {
        panic!("Tried to execute invalid data - instruction is not a supported Opcode");
    }
}
