use crate::chip8::{
    basic_types::Chip8Address, instructions::base_instruction::IsInstruction,
    machine_state::state::Chip8VMState,
};

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

    fn execute(&self, vm_state: &mut Chip8VMState) {
        vm_state.registers.set_address_register(self.address)
    }
}
