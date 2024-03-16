use crate::chip8::{
    basic_types::Byte, instructions::base_instruction::IsInstruction,
    machine_state::state::Chip8VMState,
};

#[derive(Debug)]
pub struct CopyRegisterInstruction {
    pub register_number: Byte,
    pub copied_register_number: Byte,
}

impl IsInstruction for CopyRegisterInstruction {
    fn mnemonic_name(&self) -> &'static str {
        "COPY"
    }

    fn parameters_str(&self) -> String {
        format!(
            "Vx{}, Vy{}",
            self.register_number, self.copied_register_number
        )
    }

    fn execute(&self, vm_state: &mut Chip8VMState) {
        let copy_data = vm_state
            .registers
            .get_data_register(self.copied_register_number);

        vm_state
            .registers
            .set_data_register(self.register_number, copy_data)
    }
}
