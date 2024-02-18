use crate::chip8::{basic_types::Chip8Address, instructions::base_instruction::IsInstruction};

#[derive(Debug)]
pub struct JumpInstruction {
    pub to_address: Chip8Address,
}

impl IsInstruction for JumpInstruction {
    fn full_name(&self) -> &'static str {
        "Jump"
    }

    fn mnemonic_name(&self) -> &'static str {
        "JUMP"
    }

    fn parameters_str(&self) -> String {
        format!("0x{:x}", self.to_address)
    }

    fn execute(&self) {
        todo!()
    }
}
