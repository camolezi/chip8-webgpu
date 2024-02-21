use crate::chip8::{
    basic_types::Byte, instructions::base_instruction::IsInstruction,
    machine_state::state::Chip8VMState,
};

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

    fn execute(&self, vm_state: &mut Chip8VMState) {
        vm_state
            .registers
            .set_data_register(self.register_number, self.data)
    }
}
