use crate::chip8::{basic_types::Byte, instructions::base_instruction::IsInstruction};

#[derive(Debug)]
pub struct SetRegisterInstruction {
    pub register_number: Byte,
    pub data: Byte,
}

impl IsInstruction for SetRegisterInstruction {
    fn full_name(&self) -> &'static str {
        "Set Register"
    }

    fn mnemonic_name(&self) -> &'static str {
        "SET"
    }

    fn parameters_str(&self) -> String {
        format!("V{}, {:x}", self.register_number, self.data)
    }

    fn execute(&self) {
        todo!()
    }
}
