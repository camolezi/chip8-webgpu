use crate::chip8::{
    basic_types::Byte, instructions::base_instruction::IsInstruction,
    machine_state::state::Chip8VMState,
};

#[derive(Debug)]
pub struct RegisterAddNumberInstruction {
    pub register_number: Byte,
    pub data: Byte,
}

impl IsInstruction for RegisterAddNumberInstruction {
    fn mnemonic_name(&self) -> &'static str {
        "ADDN"
    }

    fn parameters_str(&self) -> String {
        format!("V{}, {:x}", self.register_number, self.data)
    }

    fn execute(&self, vm_state: &mut Chip8VMState) {
        let register_set_data = vm_state.registers.get_data_register(self.register_number);
        let new_register_value = register_set_data.wrapping_add(self.data);

        vm_state
            .registers
            .set_data_register(self.register_number, new_register_value);
    }
}
