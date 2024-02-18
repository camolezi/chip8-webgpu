use crate::chip8::{basic_types::Chip8Address, instructions::base_instruction::IsInstruction};

#[derive(Debug)]
pub struct SetAddressRegister {
    pub address: Chip8Address,
}

impl IsInstruction for SetAddressRegister {
    fn full_name(&self) -> &'static str {
        "Set Address Register"
    }

    fn mnemonic_name(&self) -> &'static str {
        "SETI"
    }

    fn parameters_str(&self) -> String {
        format!("0x{:x}", self.address)
    }

    fn execute(&self) {
        todo!()
    }
}
