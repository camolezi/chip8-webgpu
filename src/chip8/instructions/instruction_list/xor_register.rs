use crate::chip8::{
    basic_types::Byte, instructions::base_instruction::IsInstruction,
    machine_state::state::Chip8VMState,
};

#[derive(Debug)]
pub struct XorRegisterInstruction {
    pub register_x: Byte,
    pub register_y: Byte,
}

impl IsInstruction for XorRegisterInstruction {
    fn mnemonic_name(&self) -> &'static str {
        "XOR"
    }

    fn parameters_str(&self) -> String {
        format!("Vx{}, Vy{}", self.register_x, self.register_y)
    }

    fn execute(&self, vm_state: &mut Chip8VMState) {
        let register_data_x = vm_state.registers.get_data_register(self.register_x);
        let register_data_y = vm_state.registers.get_data_register(self.register_y);

        let xor_result = register_data_x ^ register_data_y;

        vm_state
            .registers
            .set_data_register(self.register_x, xor_result);
    }
}
