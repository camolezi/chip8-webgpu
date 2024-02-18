use crate::chip8::{basic_types::Byte, instructions::base_instruction::IsInstruction};

#[derive(Debug)]
pub struct RegisterAddNumberInstruction {
    pub register_number: Byte,
    pub data: Byte,
}

impl IsInstruction for RegisterAddNumberInstruction {
    fn full_name(&self) -> &'static str {
        "Register Add Number"
    }

    fn mnemonic_name(&self) -> &'static str {
        "ADDN"
    }

    fn parameters_str(&self) -> String {
        format!("V{}, {:x}", self.register_number, self.data)
    }

    fn execute(&self) {
        todo!()
    }
}
